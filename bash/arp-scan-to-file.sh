#!/bin/bash

# Set the name of the file to current date
out_file="$(date +%D-%R)"

# Find the script's directory using the pwd command
working_dir="$(pwd)"

# Make the file according to the out_file and say hello!
touch "$working_dir/$out_file"
