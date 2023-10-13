parent_directory="."

# Loop through each repo
for repo in "$parent_directory"/*; do
    if [ -d "$repo" ]; then
        # Check if package.json exists
        if [ -f "$repo/package.json" ]; then
            echo "Installing dependencies in $repo"
            cd "$repo"
            npm install
            cd ..
        fi

        # Build to a folder
        echo "Building $repo"
        project_name=$(basename "$repo")
        output_folder="$parent_directory/www/$project_name"
        mkdir -p "$output_folder"
        
        npm run build

        # Copy output to the www directory
        cp -R "$repo/output_folder" "$output_folder"
    fi
done
