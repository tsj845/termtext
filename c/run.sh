gcc main.c -o main

if [ $? != 0 ]
then
exit 1
fi

echo "BUILT, RUNNING:"

____TTY_SETTINGS____=$(stty -g)

trap 'stty $____TTY_SETTINGS____' SIGINT EXIT

stty raw

./main $@

exit $?