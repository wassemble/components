#!/bin/bash

changed_crates=()

echo "Checking changes between HEAD^ and HEAD..."
changed_files=$(git diff --name-only HEAD^ HEAD)
echo "Changed files:"
echo "$changed_files"

echo "Checking crates directory..."
ls -la ./crates/

for crate in ./crates/*; do
  echo "Processing: $crate"
  if [ -d "$crate" ]; then
    crate_name=$(basename "$crate")
    echo "Found directory: $crate_name"
    echo "Looking for pattern: crates/$crate_name/"
    if echo "$changed_files" | grep -q "crates/$crate_name/"; then
      echo "Found changes in $crate_name"
      changed_crates+=("$crate_name")
    else
      echo "No changes found in $crate_name"
    fi
  else
    echo "Not a directory: $crate"
  fi
done

echo "Found changed crates: ${changed_crates[*]}"

if [ ${#changed_crates[@]} -eq 0 ]; then
  echo "matrix=[]" >> "$GITHUB_OUTPUT"
else
  json_array=$(printf '"%s",' "${changed_crates[@]}" | sed 's/,$//')
  echo "matrix=[$json_array]" >> "$GITHUB_OUTPUT"
fi 
