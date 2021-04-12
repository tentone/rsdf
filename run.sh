#!/bin/bash

cargo build

DIR="$(dirname "$0")"
cp -r "./src/textures" "./target/debug/textures"

cd target/debug
./rsdf