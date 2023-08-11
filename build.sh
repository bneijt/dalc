#!/bin/bash
cd "`dirname "$1"`"
set -e
rm -rf dist
trunk build --release --public-url "pr/dalc"
rsync --verbose --recursive --delete dist/ ../../../static/pr/dalc