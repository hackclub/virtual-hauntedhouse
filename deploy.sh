#!/bin/bash

submodules=(
  "wandering-wavelength=https://github.com/ShubhamPatilsd/wandering-wavelength"
  "haunted-house-testing=https://github.com/ivoinestrachan/haunted-house-testing"
  "sinrider-leaderboard=https://github.com/hackclub/sinerider-leaderboard"
)

for submodule in "${submodules[@]}"; do
  IFS='=' read -ra parts <<< "$submodule"
  submodule_name="${parts[0]}"
  submodule_url="${parts[1]}"

  # Clone submodule and deploy to Vercel
  git submodule add "$submodule_url" "rooms/$submodule_name"
  cd "rooms/$submodule_name"
  
  yes "y" | vercel --prod

  echo "hackclub" | vercel --prod

  read -p "Enter the existing project name: " project_name
  echo "$project_name" | vercel --prod

  cd ../../
done
