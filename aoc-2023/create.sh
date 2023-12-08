#!/bin/bash

directoryName="day$1"
pyFile="day$1.py"
fileName1="input.txt"
fileName2="test.txt"

# Check if the folder exists
if [ -d "$directoryName" ]; then
    echo "Folder '$directoryName' already exists. Exiting."
    exit 1
else
    # Create the folder
    echo "Creating folder '$directoryName'."
    mkdir "$directoryName"

    # Create files inside the folder
    echo "Creating files inside '$directoryName'."
    touch "$directoryName/$pyFile"
    touch "$directoryName/$fileName1"
    touch "$directoryName/$fileName2"

    echo "Folder and files created successfully."
fi
