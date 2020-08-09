#!/usr/bin/env sh

THEMES=$(i3-style --list-all  | awk 'NR>3 {print $1}')

for THEME in ${THEMES}
do
    i3-style "${THEME}" -o ~/.i3/config --reload
    scrot "${THEME}.png"
done
