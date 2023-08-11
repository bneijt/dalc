#!/bin/bash
cd "`dirname "$1"`"
set -e
rm -rf dist
trunk build --release
rsync --verbose --recursive --delete dist/ ../../../static/pr/dalc