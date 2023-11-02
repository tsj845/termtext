#!/bin/sh
# nasm -fmacho64 $1.asm && ld -macosx_version_min 10.7.0 -o $1 $1.o
# nasm -fmacho64 $1.asm && gcc *.o -o $1 -Wl,-no_pie,-v
python3 compile.py
if [ $? != 0 ]
then
echo "ERROR DURING COMPILATION"
exit 1
fi
# gcc *.o -b linkage.pout -o $1 -Wl,-no_pie
gcc *.o -o $1 -Wl,-no_pie
# exit 0


if [ $? != 0 ]
then
exit 1
fi

echo "BUILT, RUNNING:"

____TTY_SETTINGS____=$(stty -g)

trap 'stty $____TTY_SETTINGS____' SIGINT EXIT

stty raw

./$1

exit $?