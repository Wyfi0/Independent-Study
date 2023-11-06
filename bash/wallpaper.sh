#!/bin/bash

# This script uses swww, a very basic and simple wallpaper configurator,
# to run through a folder of wallpapers!

# List files in passed directory and pipe that into an awk regex filter for image files and put all that in an array!
wallpapers=($(ls $1 | awk '/[*.jpg, *.png, *.gif, *.webp, *.bmp, *.tiff]/'))

echo "You said $1"
echo "These were found in that dir: ${wallpapers[*]}"

length=${#wallpapers[@]}
echo "length is $length"

# Initialize swww
swww init

while true; do
	for paper in "${wallpapers[@]}"; do
		swww img "$1$paper"
		echo "$1$paper"
		sleep 300
	done
done
