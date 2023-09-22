#!/bin/bash

# pass the local subnet to scan
ip = $1

# Set the name of the file to current date
out_file="$(date +%m-%d-%y)"

# Find the script's directory using the pwd command
working_dir="$(pwd)"

# Make the file according to the out_file 
touch "$working_dir/$out_file"

nmap $1 > "$working_dir/$out_file"
