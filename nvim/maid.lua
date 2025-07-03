-- maid.lua - Neovim integration for Maid
-- Add to your init.lua or place in ~/.config/nvim/lua/

local maid = {}

-- Default configuration
local defaults = {
  enabled = true,
  show_notifications = true,
  autocommands = true,
  keymaps = true,
  statusline = true,
  highlight_md_changes = true,
  highlight_sh_changes = true,
  auto_suggest = true,
  paths = {}, -- Additional paths to monitor
}

local config = {}

-- Setup function with user configuration
function maid.setup(opts)
  config = vim.tbl_deep_extend("force", defaults, opts or {})
  
  if not config.enabled then
    return
  end
  
  -- Create autocommands for file monitoring
  if config.autocommands then
    maid._create_autocommands()
  end
  
  -- Create keymaps
  if config.keymaps then
    maid._create_keymaps()
  end
  
  -- Add statusline component
  if config.statusline then
    vim.g.maid_enabled = true
  end
end

-- Create autocommands for monitoring .md and .sh files
function maid._create_autocommands()
  local augroup = vim.api.nvim_create_augroup("Maid", { clear = true })
  
  -- Monitor markdown and shell script changes
  vim.api.nvim_create_autocmd({"BufWritePost"}, {
    group = augroup,
    pattern = {"*.md", "*.sh"},
    callback = function(ev)
      local filename = vim.fn.expand("%:p")
      if config.show_notifications then
        vim.notify("[Maid] Change detected: " .. filename, vim.log.levels.INFO)
      end
      
      if config.auto_suggest then
        local file_ext = vim.fn.fnamemodify(filename, ":e")
        local suggestion = ""
        
        if file_ext == "md" then
          suggestion = "Run :MaidClean to organize documentation"
        elseif file_ext == "sh" then
          suggestion = "Run :MaidClean to organize scripts"
        end
        
        if suggestion ~= "" then
          vim.defer_fn(function()
            vim.notify(suggestion, vim.log.levels.INFO)
          end, 2000)
        end
      end
    end,
  })
  
  -- Highlight AI-generated patterns in markdown files
  if config.highlight_md_changes then
    vim.api.nvim_create_autocmd({"BufEnter", "BufRead"}, {
      group = augroup,
      pattern = {"*.md"},
      callback = function()
        vim.fn.matchadd("WarningMsg", "RUBRIC", 10)
        vim.fn.matchadd("WarningMsg", "REPORT", 10)
        vim.fn.matchadd("WarningMsg", "SUMMARY", 10)
        vim.fn.matchadd("WarningMsg", "GUIDE", 10)
        vim.fn.matchadd("WarningMsg", "COMPLETE", 10)
      end,
    })
  end
end

-- Create keymaps for Maid commands
function maid._create_keymaps()
  -- Clean command
  vim.keymap.set("n", "<leader>mc", function()
    vim.cmd("!maid clean --verbose")
  end, { desc = "Maid: Clean files" })
  
  -- Keep command
  vim.keymap.set("n", "<leader>mk", function()
    vim.cmd("!maid keep --verbose")
  end, { desc = "Maid: Keep important files" })
  
  -- Show file classification
  vim.keymap.set("n", "<leader>mi", function()
    local filename = vim.fn.expand("%:p")
    vim.cmd("!maid clean --path " .. filename .. " --verbose --dry-run")
  end, { desc = "Maid: Show file classification" })
end

-- Register user commands
vim.api.nvim_create_user_command("MaidClean", function(opts)
  local args = opts.args ~= "" and opts.args or "--verbose"
  vim.cmd("!maid clean " .. args)
end, { nargs = "?", desc = "Run Maid clean command" })

vim.api.nvim_create_user_command("MaidKeep", function(opts)
  local args = opts.args ~= "" and opts.args or "--verbose"
  vim.cmd("!maid keep " .. args)
end, { nargs = "?", desc = "Run Maid keep command" })

-- Statusline component
function maid.statusline()
  if vim.g.maid_enabled then
    local ft = vim.bo.filetype
    if ft == "markdown" or ft == "sh" then
      return "🧹"
    end
  end
  return ""
end

return maid
