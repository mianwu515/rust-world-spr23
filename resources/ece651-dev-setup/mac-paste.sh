#!/bin/bash

sudo apt install xkbset

# make this happen now
setxkbmap -layout us
xkbset m
xkbset exp =m
xmodmap -e "keycode 108 = Pointer_Button2"

# and in the future
cat >> ~/.bashrc <<EOF

setxkbmap -layout us
xkbset m
xkbset exp =m
xmodmap -e "keycode 108 = Pointer_Button2"

EOF
