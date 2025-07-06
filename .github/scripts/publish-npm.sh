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
  "author": "Wassemble (https://github.com/wassemble/components)",
  "dependencies": {
    "@bytecodealliance/preview2-shim": "^0.17.2"
  },
  "description": "WebAssembly component for $1",
  "license": "Apache-2.0",
  "main": "$safe_component.js",
  "name": "@wassemble/$1",
  "repository": {
    "type": "git",
    "url": "https://github.com/wassemble/components.git"
  },
  "type": "module",
  "version": "$version"
}
EOF

# Dependencies
npm install @bytecodealliance/preview2-shim

# Generate README.md
cd ../..
wit-bindgen markdown crates/$1/wit/world.wit --html-in-md
mv $1.md npm/$1/README.md

# Transpile component
npx jco transpile $path -o npm/$1

# Publish
cd npm/$1
npm publish --access public --provenance
