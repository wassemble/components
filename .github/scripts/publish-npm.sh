#!/bin/bash

cargo component build -p $1 --release
safe_component=$(echo $1 | tr '-' '_')
path="target/wasm32-wasip1/release/$safe_component.wasm"

# Extract version from Cargo.toml
version=$(grep "^version = " "crates/$1/Cargo.toml" | cut -d'"' -f2)

rm -rf npm/$1
mkdir -p npm/$1
cd npm/$1
npm init --init-author-name "Wassemble" --init-author-url "https://github.com/wassemble/components" --init-version $version --scope=@wassemble -y
cd ../..
wit-bindgen markdown crates/$1/wit/world.wit --html-in-md
mv $1.md npm/$1/README.md
npx jco transpile $path -o npm/$1
cd npm/$1
npm publish --access public --provenance
