#!/bin/bash

echo " - Build code"
cargo build

echo " - Copy textures"
DIR="$(dirname "$0")"
cp -r "./src/textures" "./target/debug/textures"

echo " - Run code"
cd target/debug
rsdf.exe