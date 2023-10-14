parent_directory="."

for repo in "$parent_directory"/rooms/*; do
    if [ -d "$repo" ]; then
        exit_status=$?
        if [ $exit_status -eq 0 ]; then
            echo "Build succeeded for $repo"
            echo "Starting development server for $repo"
            npm run dev --prefix $repo &
        else
            echo "Build failed for $repo"
        fi
    fi
done

wait