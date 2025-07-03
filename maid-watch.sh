#!/bin/bash
# maid-watch - File monitor script for the Maid tool

if [ "$1" == "--help" ] || [ "$1" == "-h" ]; then
    echo "maid-watch - File monitor for Maid"
    echo "Usage: maid-watch [--interval=SECONDS] [--path=DIRECTORY]"
    echo ""
    echo "Options:"
    echo "  --interval=SECONDS  Set check interval in seconds (default: 60)"
    echo "  --path=DIRECTORY    Set directory to monitor (default: current directory)"
    echo "  --help, -h          Show this help message"
    exit 0
fi

# Default values
INTERVAL=60
MONITOR_PATH="."

# Parse arguments
for arg in "$@"; do
    case $arg in
        --interval=*)
        INTERVAL="${arg#*=}"
        ;;
        --path=*)
        MONITOR_PATH="${arg#*=}"
        ;;
    esac
done

echo "📡 Maid Watch - Monitoring for changes in $MONITOR_PATH"
echo "    Checking every $INTERVAL seconds"
echo "    Press Ctrl+C to stop"
echo ""

# Track files and their last modified times
declare -A last_modified

while true; do
    # Find all .md and .sh files
    while IFS= read -r file; do
        # Get file modification time
        mod_time=$(stat -f "%m" "$file" 2>/dev/null)
        
        if [ -n "$mod_time" ]; then
            # Check if we've seen this file before
            if [ -n "${last_modified[$file]}" ]; then
                # Check if modified since last check
                if [ "$mod_time" -gt "${last_modified[$file]}" ]; then
                    echo "$(date '+%H:%M:%S') 📝 [Maid] Change detected: $file"
                    
                    # Optional: Automatically suggest actions
                    filetype=$(basename "$file" | grep -q "\.md$" && echo "markdown" || echo "script")
                    echo "    Suggested action: maid clean --path $(dirname "$file") --verbose"
                fi
            fi
            
            # Update last modified time
            last_modified[$file]=$mod_time
        fi
    done < <(find "$MONITOR_PATH" -type f \( -name "*.md" -o -name "*.sh" \) 2>/dev/null)
    
    # Wait for next check
    sleep $INTERVAL
done
