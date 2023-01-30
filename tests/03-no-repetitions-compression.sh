#!/bin/sh

if [ -z "$EZLZ" -o -z "$UNEZLZ" ]
then
    echo "This script requires environment variables EZLZ and UNEZLZ be set"
    exit 1
fi

# No repetitions test: an input without repetitions should not be compressed

mkdir -p inputs compressed outputs
echo "abcdefghijklmnopqrstuvwxyz" > inputs/03

"$EZLZ" <inputs/03 >compressed/03

cmp inputs/03 compressed/03
