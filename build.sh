parent_directory="."

for repo in "$parent_directory"/rooms/*; do
    if [ -d "$repo" ]; then
        if [ -f "$repo/package.json" ]; then
            echo "Installing dependencies in $repo"

            if [ -f "$repo/yarn.lock" ]; then
                yarn install --prefix $repo
            else
                npm install --prefix $repo
            fi
        fi

        echo "Building $repo"
        npm run build --prefix $repo
    fi
done