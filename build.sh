#!/bin/bash

parent_directory="."

# Loop through each repo
for repo in "$parent_directory"/*; do
    if [ -d "$repo" ]; then
        # Check if package.json exists
        if [ -f "$repo/package.json" ]; then
            echo "Installing dependencies in $repo"
            (cd "$repo" && npm install)  # Use subshell to avoid changing the working directory
        fi

        # Build to a folder
        echo "Building $repo"
        project_name=$(basename "$repo")
        output_folder="$parent_directory/www/$project_name"
        mkdir -p "$output_folder"
        
        # Run your build command in the project folder
        (cd "$repo" && npm run build)  # Use subshell to avoid changing the working directory

    fi
done
