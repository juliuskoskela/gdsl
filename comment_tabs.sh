#!/bin/bash

# Check if the folder argument is provided
if [ -z "$1" ]; then
    echo "Please provide the folder path as an argument."
    exit 1
fi

# Get the folder path from the argument
folder="$1"

# Find Rust files (*.rs) starting with '//', '///', or '//!' and process them
while IFS= read -r -d '' file; do
    # Create a temporary file to store the modified content
    tmp_file=$(mktemp)
    
    # Read the file line by line
    while IFS= read -r line; do
        # Check if the line starts with '//'
        if [[ $line == "//"* ]]; then
            # Convert tabs to 4 spaces
            modified_line=${line//[$'\t']/    }
            echo "$modified_line" >> "$tmp_file"
        
        # Check if the line starts with '///'
        elif [[ $line == "///"* ]]; then
            # Convert tabs to 4 spaces
            modified_line=${line//[$'\t']/    }
            echo "$modified_line" >> "$tmp_file"
        
        # Check if the line starts with '//!'
        elif [[ $line == "//!"* ]]; then
            # Convert tabs to 4 spaces
            modified_line=${line//[$'\t']/    }
            echo "$modified_line" >> "$tmp_file"
        
        # If none of the above conditions are met, write the line as-is
        else
            echo "$line" >> "$tmp_file"
        fi
    done < "$file"
    
    # Replace the original file with the modified content
    mv "$tmp_file" "$file"
done < <(find "$folder" -type f -name "*.rs" -print0)
