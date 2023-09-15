#!/bin/sh
# nasm -fmacho64 $1.asm && ld -macosx_version_min 10.7.0 -o $1 $1.o
nasm -fmacho64 $1.asm && gcc $1.o -o $1 -Wl,-no_pie

if [ $? != 0 ]
then
exit 1
fi

____TTY_SETTINGS____=$(stty -g)

trap 'stty $____TTY_SETTINGS____' SIGINT EXIT

stty raw

./$1