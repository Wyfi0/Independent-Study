#!/bin/bash
if ping -c 1 192.168.0.12 &> /dev/null
then
  echo 1
else
  echo "Can't connect to Nas"
  exit 1
fi

# echo $(ssh 192.168.0.12 -p 43083 'cat /etc/hostname')
snapper -c /etc/snapper/configs/home
