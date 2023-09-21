#! /bin/bash

# this script will install all the dependancies for the project

CURRENT_DIR=$(pwd)
PARENT_DIR="$(dirname "$CURRENT_DIR")"
cd $PARENT_DIR

# if the directory uWebSocks does not exist, pull from github
if [ ! -d "uWebSockets" ]; then
    git clone --recurse-submodules https://github.com/uNetworking/uWebSockets.git
    echo "uWebSockets cloned"
fi

# if the directory uWebSocks does exist, pull from github to update
cd uWebSockets
git pull --recurse-submodules
echo "uWebSockets updated"

# go back to the DollWrapper/src directory
cd $CURRENT_DIR/DollWrapper/src

# remove the externals directory if it exists
if [ -d "externals" ]; then
    rm -rf externals
fi

# creates a directory externals
mkdir externals

# copy the needed content from uWebSockets/src to externals
cp -r $PARENT_DIR/uWebSockets/src/* externals/
# copy the needed content from uWebSockets/uSockets/src to externals
cp -r $PARENT_DIR/uWebSockets/uSockets/src/* externals/

echo "dependancies installed"