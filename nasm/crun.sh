gcc $1.c -o $1

echo "BUILT:"

if [ "$2" == "test" ]
then
echo "RUNNING:"
./$1
fi