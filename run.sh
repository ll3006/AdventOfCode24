#!/bin/bash

# Ensure script is run with enough arguments
if [ $# -lt 2 ]; then
    echo "Usage: $0 <language> <dayN> [--example]"
    exit 1
fi

LANGUAGE=$1
DAY=$2
EXAMPLE_FLAG=$3

# Determine the file to use
FILE="input.txt"
if [ "$EXAMPLE_FLAG" == "--example" ]; then
    FILE="example.txt"
fi

# Execute based on the language
case $LANGUAGE in
    python)
        SCRIPT="./python/${DAY}.py"
        INPUT="./challs/${DAY}/${FILE}"
        if [ -f "$SCRIPT" ]; then
            python "$SCRIPT" "$INPUT"
        else
            echo "Error: Python script $SCRIPT not found."
            exit 1
        fi
        ;;
    rust)
        cd ./rust || { echo "Error: Rust directory not found."; exit 1; }
        if cargo run --quiet --bin "$DAY" -- "../challs/${DAY}/${FILE}"; then
            exit 0
        else
            echo "Error: Rust binary for $DAY not found or failed to execute."
            exit 1
        fi
        ;;
    *)
        echo "Error: Unsupported language $LANGUAGE. Use 'python' or 'rust'."
        exit 1
        ;;
esac
