#!/bin/sh

if [ -z "$EZLZ" -o -z "$UNEZLZ" ]
then
    echo "This script requires environment variables EZLZ and UNEZLZ be set"
    exit 1
fi

# Repetitions test: an input with repetitions should be compressed

mkdir -p inputs compressed outputs
echo "be you, be proud of you because you can be do what we want to do" > inputs/05

"$EZLZ" <inputs/05 >compressed/05

! cmp inputs/05 compressed/05
