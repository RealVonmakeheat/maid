// maid - CLI tool for cleaning up and restructuring AI-generated files
// Copyright (C) 2025 Realvonmakeheat
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use regex::Regex;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(
    name = "maid",
    author = "Realvonmakeheat <dev@shrowd.org>",
    version = "0.1.0",
    about = "Clean up and restructure AI-generated files",
    long_about = "Maid helps clean up AI-generated .md and .sh files by renaming, reorganizing, and making them more human-readable."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Clean up AI-generated .md and .sh files
    Clean {
        /// Path to the directory to clean
        #[arg(short, long)]
        path: Option<PathBuf>,

        /// Recursively clean subdirectories
        #[arg(short, long)]
        recursive: bool,

        /// Restructure files (don't just rename)
        #[arg(short, long)]
        restructure: bool,

        /// Dry run (don't actually change anything)
        #[arg(short, long)]
        dry_run: bool,

        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },

    /// Keep important files and discard others to a temporary trash bin
    Keep {
        /// Path to the directory to process
        #[arg(short, long)]
        path: Option<PathBuf>,

        /// Recursively process subdirectories
        #[arg(short, long)]
        recursive: bool,

        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },
}

/// File types that we handle
#[derive(Debug, PartialEq)]
enum FileType {
    Markdown,
    Shell,
    Other,
}

/// The kind of document based on content analysis
#[derive(Debug)]
enum DocumentKind {
    Rubric,
    Report,
    Guide,
    Summary,
    Script,
    Unknown,
}

/// Represents a file with its metadata and classification
#[derive(Debug)]
struct FileInfo {
    path: PathBuf,
    file_type: FileType,
    doc_kind: DocumentKind,
    name: String,
    content: String,
    created_date: Option<chrono::DateTime<chrono::Local>>,
}

impl FileInfo {
    fn new(path: PathBuf) -> Result<Self> {
        let file_type = match path.extension().and_then(|ext| ext.to_str()) {
            Some("md") => FileType::Markdown,
            Some("sh") => FileType::Shell,
            _ => FileType::Other,
        };

        let name = path
            .file_stem()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown")
            .to_string();

        // Read file content
        let mut file = File::open(&path).context("Failed to open file")?;
        let mut content = String::new();
        file.read_to_string(&mut content)
            .context("Failed to read file content")?;

        // Try to determine when the file was created
        let metadata = fs::metadata(&path).ok();
        let created_date = metadata
            .and_then(|meta| meta.created().ok())
            .map(|time| chrono::DateTime::from(time));

        // Determine document kind based on content and filename
        let doc_kind = determine_document_kind(&name, &content);

        Ok(FileInfo {
            path,
            file_type,
            doc_kind,
            name,
            content,
            created_date,
        })
    }

    /// Generate a better, more human-readable filename
    fn generate_new_filename(&self) -> String {
        let normalized_name = self
            .name
            .replace('_', " ")
            .replace('-', " ")
            .to_lowercase();

        // Extract relevant information from AI-generated filenames
        let re_install = Regex::new(r"(?i)(?:setup|install)[_\-\s]*(.*?)$").ok();
        let re_test = Regex::new(r"(?i)test[_\-\s]*(.*?)$").ok();
        let re_launch = Regex::new(r"(?i)launch[_\-\s]*(.*?)$").ok();
        let re_verify = Regex::new(r"(?i)verify[_\-\s]*(.*?)$").ok();
        let re_cleanup = Regex::new(r"(?i)cleanup[_\-\s]*(.*?)$").ok();
        let re_integration = Regex::new(r"(?i)integration[_\-\s]*(.*?)$").ok();
        let re_config = Regex::new(r"(?i)config(?:uration)?[_\-\s]*(.*?)$").ok();
        let re_build = Regex::new(r"(?i)build[_\-\s]*(.*?)$").ok();
        let re_deploy = Regex::new(r"(?i)deploy[_\-\s]*(.*?)$").ok();
        
        let mut improved_name = normalized_name.clone();
        
        // Apply regex patterns to extract meaningful information
        if let Some(re) = &re_install {
            if let Some(caps) = re.captures(&normalized_name) {
                if let Some(m) = caps.get(1) {
                    improved_name = format!("Install {}", m.as_str().trim());
                }
            }
        }
        
        if let Some(re) = &re_test {
            if let Some(caps) = re.captures(&normalized_name) {
                if let Some(m) = caps.get(1) {
                    improved_name = format!("Test {}", m.as_str().trim());
                }
            }
        }
        
        if let Some(re) = &re_launch {
            if let Some(caps) = re.captures(&normalized_name) {
                if let Some(m) = caps.get(1) {
                    improved_name = format!("Launch {}", m.as_str().trim());
                }
            }
        }
        
        if let Some(re) = &re_verify {
            if let Some(caps) = re.captures(&normalized_name) {
                if let Some(m) = caps.get(1) {
                    improved_name = format!("Verify {}", m.as_str().trim());
                }
            }
        }
        
        if let Some(re) = &re_cleanup {
            if let Some(caps) = re.captures(&normalized_name) {
                if let Some(m) = caps.get(1) {
                    improved_name = format!("Cleanup {}", m.as_str().trim());
                }
            }
        }
        
        if let Some(re) = &re_integration {
            if let Some(caps) = re.captures(&normalized_name) {
                if let Some(m) = caps.get(1) {
                    improved_name = format!("Integration {}", m.as_str().trim());
                }
            }
        }
        
        if let Some(re) = &re_config {
            if let Some(caps) = re.captures(&normalized_name) {
                if let Some(m) = caps.get(1) {
                    improved_name = format!("Configuration {}", m.as_str().trim());
                }
            }
        }
        
        if let Some(re) = &re_build {
            if let Some(caps) = re.captures(&normalized_name) {
                if let Some(m) = caps.get(1) {
                    improved_name = format!("Build {}", m.as_str().trim());
                }
            }
        }
        
        if let Some(re) = &re_deploy {
            if let Some(caps) = re.captures(&normalized_name) {
                if let Some(m) = caps.get(1) {
                    improved_name = format!("Deploy {}", m.as_str().trim());
                }
            }
        }

        // Convert to title case
        let title_case: String = improved_name
            .split_whitespace()
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => {
                        let capitalized = first.to_uppercase().collect::<String>();
                        capitalized + chars.as_str()
                    }
                }
            })
            .collect::<Vec<String>>()
            .join(" ");

        // Add prefix based on document kind
        let prefixed_name = match self.doc_kind {
            DocumentKind::Rubric => format!("Rubric - {}", title_case),
            DocumentKind::Report => format!("Report - {}", title_case),
            DocumentKind::Guide => format!("Guide - {}", title_case),
            DocumentKind::Summary => format!("Summary - {}", title_case),
            DocumentKind::Script => title_case,
            DocumentKind::Unknown => title_case,
        };

        // Add extension
        match self.file_type {
            FileType::Markdown => format!("{}.md", prefixed_name),
            FileType::Shell => format!("{}.sh", prefixed_name),
            FileType::Other => self
                .path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("unknown")
                .to_string(),
        }
    }

    /// Generate suggested target directory based on document kind
    fn suggest_target_directory(&self, base_dir: &Path) -> PathBuf {
        match (&self.file_type, &self.doc_kind) {
            (FileType::Markdown, DocumentKind::Rubric) => base_dir.join("docs/rubrics"),
            (FileType::Markdown, DocumentKind::Report) => base_dir.join("docs/reports"),
            (FileType::Markdown, DocumentKind::Guide) => base_dir.join("docs/guides"),
            (FileType::Markdown, DocumentKind::Summary) => base_dir.join("docs/summaries"),
            (FileType::Shell, DocumentKind::Script) => {
                // Determine script subcategory
                let content_lower = self.content.to_lowercase();
                if content_lower.contains("install") || content_lower.contains("setup") {
                    base_dir.join("scripts/setup")
                } else if content_lower.contains("test") {
                    base_dir.join("scripts/tests")
                } else if content_lower.contains("build") {
                    base_dir.join("scripts/build")
                } else {
                    base_dir.join("scripts")
                }
            }
            _ => base_dir.to_path_buf(),
        }
    }
}

/// Determine document kind based on filename and content
fn determine_document_kind(filename: &str, content: &str) -> DocumentKind {
    let filename_lower = filename.to_lowercase();
    let content_lower = content.to_lowercase();
    
    // Check for rubrics
    if filename_lower.contains("rubric") 
        || content_lower.contains("# rubric") 
        || content_lower.contains("rubric for") 
        || content_lower.contains("evaluation rubric") 
        || content_lower.contains("assessment criteria") 
        || content_lower.contains("scoring guide") {
        return DocumentKind::Rubric;
    }
    
    // Check for reports
    if filename_lower.contains("report") 
        || filename_lower.contains("complete") 
        || filename_lower.contains("status") 
        || filename_lower.contains("analysis") 
        || filename_lower.contains("assessment") 
        || content_lower.contains("# report") 
        || content_lower.contains("# completion") 
        || content_lower.contains("# status") 
        || content_lower.contains("# analysis") 
        || content_lower.contains("task completion") 
        || content_lower.contains("completion report") 
        || content_lower.contains("status update") {
        return DocumentKind::Report;
    }
    
    // Check for guides
    if filename_lower.contains("guide") 
        || filename_lower.contains("how_to") 
        || filename_lower.contains("howto") 
        || filename_lower.contains("manual") 
        || filename_lower.contains("tutorial") 
        || filename_lower.contains("instructions") 
        || content_lower.contains("# guide") 
        || content_lower.contains("# how to") 
        || content_lower.contains("step by step") 
        || content_lower.contains("# tutorial") 
        || content_lower.contains("# instructions") 
        || content_lower.contains("how to use") 
        || content_lower.contains("usage instructions") {
        return DocumentKind::Guide;
    }
    
    // Check for summaries
    if filename_lower.contains("summary") 
        || filename_lower.contains("overview") 
        || filename_lower.contains("recap") 
        || filename_lower.contains("synopsis") 
        || content_lower.contains("# summary") 
        || content_lower.contains("## summary") 
        || content_lower.contains("# overview") 
        || content_lower.contains("# recap") 
        || content_lower.contains("in conclusion") 
        || content_lower.contains("executive summary") 
        || content_lower.contains("project summary") {
        return DocumentKind::Summary;
    }
    
    // Check for scripts (shell files are automatically scripts)
    if filename_lower.ends_with(".sh") {
        return DocumentKind::Script;
    }
    
    // Default
    DocumentKind::Unknown
}

/// Process a single file
fn process_file(
    file_path: &Path,
    base_dir: &Path,
    restructure: bool,
    dry_run: bool,
    verbose: bool,
) -> Result<()> {
    // Skip if not a file or if hidden
    if !file_path.is_file() || is_hidden(file_path) {
        return Ok(());
    }

    // Process only markdown and shell files
    let extension = file_path.extension().and_then(|ext| ext.to_str());
    if !matches!(extension, Some("md") | Some("sh")) {
        return Ok(());
    }

    // Handle symlinks
    let real_path = if file_path.is_symlink() {
        fs::read_link(file_path).unwrap_or_else(|_| file_path.to_path_buf())
    } else {
        file_path.to_path_buf()
    };

    // Analyze the file
    let file_info = match FileInfo::new(real_path.clone()) {
        Ok(info) => info,
        Err(e) => {
            if verbose {
                println!(
                    "{} {} - Error: {}",
                    "Skipping:".red().bold(),
                    file_path.display().to_string().yellow(),
                    e.to_string()
                );
            }
            return Ok(());
        }
    };
    
    // Generate new filename
    let new_filename = file_info.generate_new_filename();
    
    // Determine target location
    let target_dir = if restructure {
        file_info.suggest_target_directory(base_dir)
    } else {
        file_path.parent().unwrap_or(Path::new(".")).to_path_buf()
    };
    
    let target_path = target_dir.join(&new_filename);
    
    // Print what we're doing
    if verbose {
        println!(
            "{} {} -> {}",
            "Processing:".cyan().bold(),
            file_path.display().to_string().yellow(),
            target_path.display().to_string().green()
        );
        println!(
            "  {} {}",
            "Type:".cyan(),
            format!("{:?}", file_info.doc_kind).magenta()
        );
    } else {
        print!(".");
        io::stdout().flush()?;
    }
    
    // If not dry run, perform the operation
    if !dry_run {
        // Create target directory if it doesn't exist
        if !target_dir.exists() {
            fs::create_dir_all(&target_dir)?;
        }
        
        // Check if source and target are the same
        let source_canonical = fs::canonicalize(file_path).ok();
        let target_canonical_exists = target_path.exists() && fs::canonicalize(&target_path).is_ok();
        let target_canonical = if target_canonical_exists {
            fs::canonicalize(&target_path).ok()
        } else {
            None
        };
        
        // Skip if target already exists and is the same file
        if source_canonical == target_canonical && target_canonical.is_some() {
            if verbose {
                println!(
                    "  {} {}",
                    "Skip:".yellow(),
                    "Source and target are the same file".bright_black()
                );
            }
            return Ok(());
        }
        
        if target_path.exists() {
            if verbose {
                println!(
                    "  {} {}",
                    "Skip:".yellow(),
                    "Target file already exists".bright_black()
                );
            }
            return Ok(());
        }
        
        // Copy the file - use a symlink for symlinks, copy for real files
        if file_path.is_symlink() {
            if verbose {
                println!("  {} {}", "Info:".blue(), "Creating symlink");
            }
            
            #[cfg(unix)]
            {
                use std::os::unix::fs::symlink;
                let original_target = fs::read_link(file_path)?;
                symlink(original_target, &target_path)?;
            }
            
            #[cfg(windows)]
            {
                use std::os::windows::fs::{symlink_file, symlink_dir};
                let original_target = fs::read_link(file_path)?;
                if original_target.is_file() {
                    symlink_file(original_target, &target_path)?;
                } else {
                    symlink_dir(original_target, &target_path)?;
                }
            }
        } else {
            // Regular file copy
            fs::copy(file_path, &target_path)?;
        }
        
        if verbose {
            println!(
                "  {} {}",
                "Success:".green(),
                "File processed".bright_green()
            );
        }
    }
    
    Ok(())
}

/// Check if a file is hidden
fn is_hidden(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.starts_with('.'))
        .unwrap_or(false)
}

/// Clean up files in a directory
fn clean_directory(
    dir_path: &Path,
    recursive: bool,
    restructure: bool,
    dry_run: bool,
    verbose: bool,
) -> Result<()> {
    // Count all files
    let mut processed_files = 0;
    let mut skipped_files = 0;
    let mut md_files = 0;
    let mut sh_files = 0;
    
    // Count files first for progress bar
    let file_paths = if recursive {
        WalkDir::new(dir_path)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type().is_file())
            .filter(|entry| {
                let path = entry.path();
                let ext = path.extension().and_then(|ext| ext.to_str());
                matches!(ext, Some("md") | Some("sh"))
            })
            .map(|entry| entry.path().to_path_buf())
            .collect::<Vec<_>>()
    } else {
        fs::read_dir(dir_path)
            .context("Failed to read directory")?
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type().map(|ft| ft.is_file()).unwrap_or(false))
            .map(|entry| entry.path())
            .filter(|path| {
                let ext = path.extension().and_then(|ext| ext.to_str());
                matches!(ext, Some("md") | Some("sh"))
            })
            .collect::<Vec<_>>()
    };
    
    let total_files = file_paths.len();
        
    println!(
        "{} {} files in {}{}",
        "Found".cyan().bold(),
        total_files.to_string().yellow().bold(),
        dir_path.display().to_string().green(),
        if dry_run {
            " (DRY RUN)".bright_red().bold().to_string()
        } else {
            "".to_string()
        }
    );
    
    let progress_bar = if !verbose {
        let pb = ProgressBar::new(total_files as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
                .unwrap()
                .progress_chars("#>-"),
        );
        Some(pb)
    } else {
        None
    };
    
    for file_path in file_paths {
        // Update file type counts
        if let Some(ext) = file_path.extension().and_then(|ext| ext.to_str()) {
            match ext {
                "md" => md_files += 1,
                "sh" => sh_files += 1,
                _ => {}
            }
        }
        
        match process_file(&file_path, dir_path, restructure, dry_run, verbose) {
            Ok(()) => {
                processed_files += 1;
            }
            Err(e) => {
                skipped_files += 1;
                if verbose {
                    println!(
                        "{} {} - Error: {}",
                        "Error:".red().bold(),
                        file_path.display().to_string().yellow(),
                        e.to_string()
                    );
                }
            }
        }
        
        if let Some(pb) = &progress_bar {
            pb.inc(1);
        }
    }
    
    if let Some(pb) = progress_bar {
        pb.finish_with_message("Done!");
        println!(); // Add a blank line after the progress bar
    }
    
    // Print a summary
    println!("\n{}", "📊 Summary".cyan().bold());
    println!("  {} {}", "Total files found:".bright_white(), total_files);
    println!("  {} {}", "Files processed:".green(), processed_files);
    println!("  {} {}", "Files skipped:".yellow(), skipped_files);
    println!("  {} {}", "Markdown files:".magenta(), md_files);
    println!("  {} {}", "Shell scripts:".magenta(), sh_files);
    
    Ok(())
}

/// Structure to track file analysis results for keep command
struct KeepAnalysis {
    important_files: Vec<PathBuf>,
    redundant_files: Vec<PathBuf>,
    trash_dir: PathBuf,
}

impl KeepAnalysis {
    fn new() -> Self {
        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();
        let trash_dir = PathBuf::from(format!("/tmp/maid-trash-bin-{}", timestamp));
        
        KeepAnalysis {
            important_files: Vec::new(),
            redundant_files: Vec::new(),
            trash_dir,
        }
    }
    
    /// Evaluate files to determine which ones should be kept
    fn evaluate_files(&mut self, file_paths: &[PathBuf], verbose: bool) -> Result<()> {
        // Group files by document kind
        let mut rubrics = Vec::new();
        let mut reports = Vec::new();
        let mut guides = Vec::new();
        let mut summaries = Vec::new();
        let mut scripts = Vec::new();
        
        for file_path in file_paths {
            match FileInfo::new(file_path.clone()) {
                Ok(info) => {
                    match info.doc_kind {
                        DocumentKind::Rubric => rubrics.push((file_path.clone(), info)),
                        DocumentKind::Report => reports.push((file_path.clone(), info)),
                        DocumentKind::Guide => guides.push((file_path.clone(), info)),
                        DocumentKind::Summary => summaries.push((file_path.clone(), info)),
                        DocumentKind::Script => scripts.push((file_path.clone(), info)),
                        DocumentKind::Unknown => {
                            // For unknown types, keep them by default
                            self.important_files.push(file_path.clone());
                        }
                    }
                },
                Err(_) => {
                    // If we can't analyze the file, keep it by default
                    self.important_files.push(file_path.clone());
                }
            }
        }
        
        // Keep the most comprehensive rubric, discard others
        if !rubrics.is_empty() {
            // Find the most comprehensive rubric (highest word count as a simple heuristic)
            rubrics.sort_by(|(_, a), (_, b)| {
                let a_words = a.content.split_whitespace().count();
                let b_words = b.content.split_whitespace().count();
                b_words.cmp(&a_words)
            });
            
            // Keep the most comprehensive one
            if let Some((path, _)) = rubrics.first() {
                self.important_files.push(path.clone());
                
                if verbose {
                    println!(
                        "{} {} (most comprehensive rubric)",
                        "Keeping:".green().bold(),
                        path.display().to_string().green()
                    );
                }
            }
            
            // Mark others as redundant
            for (path, _) in rubrics.iter().skip(1) {
                self.redundant_files.push(path.clone());
                
                if verbose {
                    println!(
                        "{} {} (redundant rubric)",
                        "Discarding:".yellow().bold(),
                        path.display().to_string().yellow()
                    );
                }
            }
        }
        
        // For reports, keep the most recent ones
        if !reports.is_empty() {
            // Sort by creation date if available, newest first
            reports.sort_by(|(_, a), (_, b)| {
                match (&a.created_date, &b.created_date) {
                    (Some(a_date), Some(b_date)) => b_date.cmp(a_date),
                    (Some(_), None) => std::cmp::Ordering::Less,
                    (None, Some(_)) => std::cmp::Ordering::Greater,
                    (None, None) => std::cmp::Ordering::Equal,
                }
            });
            
            // Keep the newest report
            if let Some((path, _)) = reports.first() {
                self.important_files.push(path.clone());
                
                if verbose {
                    println!(
                        "{} {} (most recent report)",
                        "Keeping:".green().bold(),
                        path.display().to_string().green()
                    );
                }
            }
            
            // Mark older reports as redundant
            for (path, _) in reports.iter().skip(1) {
                self.redundant_files.push(path.clone());
                
                if verbose {
                    println!(
                        "{} {} (older report)",
                        "Discarding:".yellow().bold(),
                        path.display().to_string().yellow()
                    );
                }
            }
        }
        
        // Keep all guides
        for (path, _) in guides {
            self.important_files.push(path.clone());
            
            if verbose {
                println!(
                    "{} {} (guide)",
                    "Keeping:".green().bold(),
                    path.display().to_string().green()
                );
            }
        }
        
        // Keep the most recent summary, discard others
        if !summaries.is_empty() {
            // Sort by creation date if available, newest first
            summaries.sort_by(|(_, a), (_, b)| {
                match (&a.created_date, &b.created_date) {
                    (Some(a_date), Some(b_date)) => b_date.cmp(a_date),
                    (Some(_), None) => std::cmp::Ordering::Less,
                    (None, Some(_)) => std::cmp::Ordering::Greater,
                    (None, None) => std::cmp::Ordering::Equal,
                }
            });
            
            // Keep the newest summary
            if let Some((path, _)) = summaries.first() {
                self.important_files.push(path.clone());
                
                if verbose {
                    println!(
                        "{} {} (most recent summary)",
                        "Keeping:".green().bold(),
                        path.display().to_string().green()
                    );
                }
            }
            
            // Mark older summaries as redundant
            for (path, _) in summaries.iter().skip(1) {
                self.redundant_files.push(path.clone());
                
                if verbose {
                    println!(
                        "{} {} (older summary)",
                        "Discarding:".yellow().bold(),
                        path.display().to_string().yellow()
                    );
                }
            }
        }
        
        // Keep all scripts but analyze them for duplication
        let mut unique_scripts: Vec<(PathBuf, FileInfo)> = Vec::new();
        
        for (path, info) in scripts {
            // Simplistic content-based deduplication
            let mut is_duplicate = false;
            
            for (existing_path, existing_info) in &unique_scripts {
                if info.content.trim() == existing_info.content.trim() {
                    if verbose {
                        println!(
                            "{} {} (duplicate of {})",
                            "Discarding:".yellow().bold(),
                            path.display().to_string().yellow(),
                            existing_path.display().to_string().green()
                        );
                    }
                    
                    self.redundant_files.push(path.clone());
                    is_duplicate = true;
                    break;
                }
            }
            
            if !is_duplicate {
                unique_scripts.push((path.clone(), info));
                self.important_files.push(path.clone());
                
                if verbose {
                    println!(
                        "{} {} (unique script)",
                        "Keeping:".green().bold(),
                        path.display().to_string().green()
                    );
                }
            }
        }
        
        Ok(())
    }
    
    /// Move redundant files to the trash bin
    fn move_to_trash(&self, verbose: bool) -> Result<()> {
        if self.redundant_files.is_empty() {
            return Ok(());
        }
        
        // Create trash directory
        fs::create_dir_all(&self.trash_dir)?;
        
        // Move redundant files to trash
        for file_path in &self.redundant_files {
            let file_name = file_path
                .file_name()
                .unwrap_or_else(|| std::ffi::OsStr::new("unknown"))
                .to_string_lossy();
                
            let target_path = self.trash_dir.join(file_name.to_string());
            
            // Handle duplicate file names in trash
            let mut actual_target_path = target_path.clone();
            let mut counter = 1;
            
            while actual_target_path.exists() {
                let new_name = format!(
                    "{}-{}.{}",
                    target_path.file_stem().unwrap().to_string_lossy(),
                    counter,
                    target_path.extension().unwrap_or_default().to_string_lossy()
                );
                actual_target_path = self.trash_dir.join(new_name);
                counter += 1;
            }
            
            fs::rename(file_path, &actual_target_path)?;
            
            if verbose {
                println!(
                    "{} {} -> {}",
                    "Moved:".yellow().bold(),
                    file_path.display().to_string().yellow(),
                    actual_target_path.display().to_string().bright_black()
                );
            }
        }
        
        // Set up self-destruct on terminal close
        // We'll create a script that deletes the trash bin
        let script_path = self.trash_dir.join("self_destruct.sh");
        let script_content = format!(
            r#"#!/bin/bash
# This script will delete the maid trash bin when the terminal session ends
trap "rm -rf {}" EXIT
# Keep the terminal session open until explicit termination
cat <(echo "Maid trash bin will be deleted when this terminal is closed.")
# Execute the trap even if the script is killed
exec bash"#,
            self.trash_dir.display()
        );
        
        let mut file = File::create(&script_path)?;
        file.write_all(script_content.as_bytes())?;
        
        // Make the script executable
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&script_path)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&script_path, perms)?;
        }
        
        // Launch the self-destruct script in a new terminal
        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            Command::new("open")
                .args(["-a", "Terminal", script_path.to_str().unwrap()])
                .spawn()?;
        }
        
        #[cfg(target_os = "linux")]
        {
            use std::process::Command;
            Command::new("x-terminal-emulator")
                .args(["-e", script_path.to_str().unwrap()])
                .spawn()?;
        }
        
        #[cfg(target_os = "windows")]
        {
            use std::process::Command;
            Command::new("cmd")
                .args(["/c", "start", "cmd", "/k", script_path.to_str().unwrap()])
                .spawn()?;
        }
        
        Ok(())
    }
    
    /// Generate a comprehensive rubric based on kept files
    fn generate_comprehensive_rubric(&self, base_dir: &Path, verbose: bool) -> Result<()> {
        if self.important_files.is_empty() {
            if verbose {
                println!("{} No files to analyze for rubric generation", "Warning:".yellow().bold());
            }
            return Ok(());
        }
        
        if verbose {
            println!("{} Generating comprehensive rubric...", "Info:".blue().bold());
        }
        
        let mut rubric_content = String::new();
        rubric_content.push_str("# Comprehensive Project Rubric\n\n");
        rubric_content.push_str("*Generated by Maid - AI-generated file organizer*\n\n");
        rubric_content.push_str("## Overview\n\n");
        rubric_content.push_str("This rubric is automatically generated based on the analysis of project documentation and scripts.\n\n");
        
        // Add current date
        let current_date = chrono::Local::now().format("%Y-%m-%d").to_string();
        rubric_content.push_str(&format!("Generated on: {}\n\n", current_date));
        
        // Extract key concepts from important files
        rubric_content.push_str("## Key Project Components\n\n");
        
        let mut keywords = std::collections::HashMap::new();
        
        for file_path in &self.important_files {
            if let Ok(info) = FileInfo::new(file_path.clone()) {
                // Extract keywords from content
                let content_words: Vec<&str> = info.content
                    .split(|c: char| !c.is_alphanumeric() && c != '_')
                    .filter(|s| !s.is_empty() && s.len() > 3)
                    .collect();
                    
                for word in content_words {
                    *keywords.entry(word.to_lowercase()).or_insert(0) += 1;
                }
            }
        }
        
        // Get top keywords
        let mut keyword_counts: Vec<(String, usize)> = keywords.into_iter().collect();
        keyword_counts.sort_by(|a, b| b.1.cmp(&a.1));
        
        let top_keywords: Vec<String> = keyword_counts
            .iter()
            .take(15)
            .map(|(word, _)| word.clone())
            .collect();
            
        rubric_content.push_str("### Key Terms\n\n");
        for keyword in &top_keywords {
            rubric_content.push_str(&format!("- {}\n", keyword));
        }
        
        rubric_content.push_str("\n## Evaluation Criteria\n\n");
        
        // Add sections based on file types we've kept
        let mut has_documentation = false;
        let mut has_scripts = false;
        
        for file_path in &self.important_files {
            if let Ok(info) = FileInfo::new(file_path.clone()) {
                match info.file_type {
                    FileType::Markdown => has_documentation = true,
                    FileType::Shell => has_scripts = true,
                    _ => {}
                }
            }
        }
        
        if has_documentation {
            rubric_content.push_str("### Documentation Quality\n\n");
            rubric_content.push_str("| Criterion | Poor | Satisfactory | Excellent |\n");
            rubric_content.push_str("|-----------|------|--------------|----------|\n");
            rubric_content.push_str("| Completeness | Documentation missing key components | Most features documented | Comprehensive documentation of all features |\n");
            rubric_content.push_str("| Clarity | Confusing or unclear | Generally clear with some issues | Clear, concise, and well-organized |\n");
            rubric_content.push_str("| Examples | Few or no examples | Some examples provided | Rich examples covering typical use cases |\n");
            rubric_content.push_str("\n");
        }
        
        if has_scripts {
            rubric_content.push_str("### Script Quality\n\n");
            rubric_content.push_str("| Criterion | Poor | Satisfactory | Excellent |\n");
            rubric_content.push_str("|-----------|------|--------------|----------|\n");
            rubric_content.push_str("| Functionality | Scripts fail to accomplish tasks | Scripts work but have limitations | Scripts work flawlessly for all use cases |\n");
            rubric_content.push_str("| Readability | Poorly commented and structured | Adequate comments and structure | Well-commented, clear structure |\n");
            rubric_content.push_str("| Error Handling | Little or no error handling | Basic error handling | Comprehensive error handling with helpful messages |\n");
            rubric_content.push_str("\n");
        }
        
        // Add references to hallucination issues
        rubric_content.push_str("## Note on Documentation Management\n\n");
        rubric_content.push_str("Research has shown that having too many redundant documentation files can lead to confusion and AI hallucinations when used as reference material. This rubric is generated as part of an effort to consolidate and organize project documentation.\n\n");
        rubric_content.push_str("### References\n\n");
        rubric_content.push_str("- Hallucination in Large Language Models: [https://arxiv.org/abs/2309.01219](https://arxiv.org/abs/2309.01219)\n");
        rubric_content.push_str("- The Impact of Contradictory Data on AI Training: [https://www.nature.com/articles/s41467-023-42879-y](https://www.nature.com/articles/s41467-023-42879-y)\n");
        
        // Save rubric to file
        let rubric_path = base_dir.join("COMPREHENSIVE_PROJECT_RUBRIC.md");
        let mut file = File::create(&rubric_path)?;
        file.write_all(rubric_content.as_bytes())?;
        
        if verbose {
            println!(
                "{} {}",
                "Created:".green().bold(),
                rubric_path.display().to_string().green()
            );
        }
        
        Ok(())
    }
}

/// Keep important files and move others to trash
fn keep_important_files(
    dir_path: &Path,
    recursive: bool,
    verbose: bool,
) -> Result<()> {
    // Find all markdown and shell files
    let file_paths = if recursive {
        WalkDir::new(dir_path)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type().is_file())
            .filter(|entry| {
                let path = entry.path();
                let ext = path.extension().and_then(|ext| ext.to_str());
                matches!(ext, Some("md") | Some("sh"))
            })
            .map(|entry| entry.path().to_path_buf())
            .collect::<Vec<_>>()
    } else {
        fs::read_dir(dir_path)
            .context("Failed to read directory")?
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type().map(|ft| ft.is_file()).unwrap_or(false))
            .map(|entry| entry.path())
            .filter(|path| {
                let ext = path.extension().and_then(|ext| ext.to_str());
                matches!(ext, Some("md") | Some("sh"))
            })
            .collect::<Vec<_>>()
    };
    
    let total_files = file_paths.len();
    
    println!(
        "{} {} files in {}",
        "Found".cyan().bold(),
        total_files.to_string().yellow().bold(),
        dir_path.display().to_string().green(),
    );
    
    if total_files == 0 {
        println!("{} No files to process", "Warning:".yellow().bold());
        return Ok(());
    }
    
    // Create and run the analysis
    let mut analysis = KeepAnalysis::new();
    analysis.evaluate_files(&file_paths, verbose)?;
    
    // Generate statistics
    let important_count = analysis.important_files.len();
    let redundant_count = analysis.redundant_files.len();
    
    println!("\n{}", "📊 Analysis Results".cyan().bold());
    println!("  {} {}", "Files to keep:".green(), important_count);
    println!("  {} {}", "Files to move to trash:".yellow(), redundant_count);
    
    // Confirm with the user
    print!("\n{} This will move {} files to the trash bin. Continue? (y/N) ", 
        "⚠️".yellow().bold(), 
        redundant_count.to_string().yellow().bold()
    );
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    if !matches!(input.trim().to_lowercase().as_str(), "y" | "yes") {
        println!("{} Operation cancelled", "Info:".blue().bold());
        return Ok(());
    }
    
    // Move redundant files to trash
    analysis.move_to_trash(verbose)?;
    
    // Generate comprehensive rubric
    analysis.generate_comprehensive_rubric(dir_path, verbose)?;
    
    // Print summary
    println!("\n{}", "📊 Summary".cyan().bold());
    println!("  {} {}", "Files kept:".green(), important_count);
    println!("  {} {}", "Files moved to trash:".yellow(), redundant_count);
    println!(
        "  {} {}",
        "Trash location:".bright_black(),
        analysis.trash_dir.display().to_string().bright_black()
    );
    println!("  {} The trash bin will be automatically deleted when you close its terminal window", 
        "Note:".blue().bold()
    );
    
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Clean {
            path,
            recursive,
            restructure,
            dry_run,
            verbose,
        } => {
            let dir_path = path.unwrap_or_else(|| PathBuf::from("."));
            
            println!(
                "{} {}",
                "Maid".bright_cyan().bold(),
                "is cleaning up your AI-generated files...".bright_white()
            );
            
            if !dir_path.exists() {
                anyhow::bail!("Directory does not exist: {}", dir_path.display());
            }
            
            if !dir_path.is_dir() {
                anyhow::bail!("Not a directory: {}", dir_path.display());
            }
            
            clean_directory(&dir_path, recursive, restructure, dry_run, verbose)?;
            
            println!(
                "\n{} {} {}\n",
                "✨".bright_yellow(),
                "Cleaning complete!".green().bold(),
                "✨".bright_yellow()
            );
        }
        Commands::Keep {
            path,
            recursive,
            verbose,
        } => {
            let dir_path = path.unwrap_or_else(|| PathBuf::from("."));
            
            println!(
                "{} {}",
                "Maid".bright_cyan().bold(),
                "is keeping your important files safe...".bright_white()
            );
            
            if !dir_path.exists() {
                anyhow::bail!("Directory does not exist: {}", dir_path.display());
            }
            
            if !dir_path.is_dir() {
                anyhow::bail!("Not a directory: {}", dir_path.display());
            }
            
            keep_important_files(&dir_path, recursive, verbose)?;
            
            println!(
                "\n{} {} {}\n",
                "✨".bright_yellow(),
                "Operation complete!".green().bold(),
                "✨".bright_yellow()
            );
        }
    }
    
    Ok(())
}
