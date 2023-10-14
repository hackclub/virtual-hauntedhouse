parent_directory="."

for repo in "$parent_directory"/*; do
    if [ -d "$repo" ]; then
        if [ -f "$repo/package.json" ]; then
            echo "Installing dependencies in $repo"

            if [ -f "$repo/yarn.lock" ]; then
                (cd "$repo" && yarn install)
            else
                (cd "$repo" && npm install)
            fi
        fi

        project_name=$(basename "$repo")
        output_folder="$parent_directory/www/$project_name"

        if [ ! -d "$output_folder" ]; then
            echo "Building $repo"
            mkdir -p "$output_folder"
            cd "$repo" && npm run build
            exit_status=$?
            if [ $exit_status -eq 0 ]; then
                echo "Build succeeded for $repo"
            else
                echo "Build failed for $repo"
            fi
        fi
    fi
done
