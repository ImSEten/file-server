#!/bin/bash

# Helper script for Github Actions to install protobuf on different runners.
echo "OS: $RUNNER_OS"

if [ "$RUNNER_OS" == 'Linux' ]; then
    # Install on Linux
    sudo apt-get update
    sudo apt-get install -y protobuf-compiler
    sudo apt-get install -y libssl-dev
elif [ "$RUNNER_OS" == 'macOS' ]; then
    # Install on macOS
    brew install protobuf
    brew install libssl-dev
elif [ "$RUNNER_OS" == 'Windows' ]; then
    # Install on Windows
    choco install -y protoc
    choco install -u libssl-dev
else
    echo "Unsupported OS: $RUNNER_OS"
    exit 1
fi

# Check the installed Protobuf version
protoc --version
