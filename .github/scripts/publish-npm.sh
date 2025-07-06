#!/bin/bash

# Install jco
npm install -g @bytecodealliance/jco

# Variables
safe_component=$(echo $1 | tr '-' '_')
path="target/wasm32-wasip1/release/$safe_component.wasm"
version=$(grep "^version = " "crates/$1/Cargo.toml" | cut -d'"' -f2)

# Clean up
rm -rf npm/$1
mkdir -p npm/$1
cd npm/$1

# Create package.json
cat > package.json << EOF
{
  "name": "@wassemble/$1",
  "version": "$version",
  "main": "index.js",
  "repository": {
    "type": "git",
    "url": "https://github.com/wassemble/components.git"
  },
  "author": "Wassemble (https://github.com/wassemble/components)",
  "license": "Apache-2.0",
  "description": "WebAssembly component for $1"
}
EOF

# Generate README.md
cd ../..
wit-bindgen markdown crates/$1/wit/world.wit --html-in-md
mv $1.md npm/$1/README.md

# Transpile component
npx jco transpile $path -o npm/$1

# Publish
cd npm/$1
npm publish --access public --provenance
