#!/bin/sh

if [ -z "$EZLZ" -o -z "$UNEZLZ" ]
then
    echo "This script requires environment variables EZLZ and UNEZLZ be set"
    exit 1
fi

# Identity test: does decompression bring back the initial file

mkdir -p inputs compressed outputs
echo "be you, be proud of you because you can be do what we want to do" > inputs/04

"$EZLZ" <inputs/04 >compressed/04
"$UNEZLZ" <compressed/04 >outputs/04

cmp inputs/04 outputs/04
