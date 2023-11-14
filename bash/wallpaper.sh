#!/bin/bash

# This script uses swww, a very basic and simple wallpaper configurator,
# to run through a folder of wallpapers!

# List files in passed directory and pipe that into an awk regex filter for image files and put all that in an array!
wallpapers=($(ls $1 | awk '/[*.jpg, *.png, *.gif, *.webp, *.bmp, *.tiff]/'))

echo "You said $1"
echo "These were found in that dir: ${wallpapers[*]}"

good=false

length=${#wallpapers[@]}
echo "$length images"

# Before running anything else make sure both passed variables are not null
if [ -z $1 ] || [ -z $2 ]; then
	good=false
	echo "You need to pass the directory of your wallpapers and then the amount of seconds you want each wallpaper to be there!"
else
	# and tell the while loop that were good to go!
	swww init
	good=true
fi

# Run this continually
while [ "$good" = true ]; do
	# For every item in the wallpaper list
	for paper in "${wallpapers[@]}"; do
		# Set image to the directory then  the filename
		swww img "$1$paper"
		# Print out the directory then the filename
		echo "$1$paper"
		# Sleep for the second passed variable
		sleep $2
	done
done
