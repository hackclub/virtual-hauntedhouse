submodules=(
  "wandering-wavelength=https://github.com/ShubhamPatilsd/wandering-wavelength"
  "haunted-house-testing=https://github.com/ivoinestrachan/haunted-house-testing"
)

for submodule in "${submodules[@]}"; do
  IFS='=' read -ra parts <<< "$submodule"
  submodule_name="${parts[0]}"
  submodule_url="${parts[1]}"

  git submodule add "$submodule_url" "rooms/$submodule_name"
  cd "rooms/$submodule_name"
  

  echo "y" | vercel --prod
  echo "sikethedev" | vercel --prod  
  
  echo "$project_name" | vercel --prod

  cd ../../
done
