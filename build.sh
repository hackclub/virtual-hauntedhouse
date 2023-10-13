parent_directory="."


vercel_project_name="hauntedhouse-docker"

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
        
        # Run your build command in the project folder
        cd "$repo"
        npm run build
        cd ..

        # Deploy to Vercel
        echo "Deploying $repo to Vercel"
        vercel --prod --confirm --token $VERCEL_TOKEN --scope $VERCEL_TEAM_NAME --project $vercel_project_name "$output_folder"
    fi
done
