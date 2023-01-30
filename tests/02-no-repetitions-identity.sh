#!/bin/sh

if [ -z "$EZLZ" -o -z "$UNEZLZ" ]
then
    echo "This script requires environment variables EZLZ and UNEZLZ be set"
    exit 1
fi

# Identity test: does decompression bring back the initial file

mkdir -p inputs compressed outputs
echo "abcdefghijklmnopqrstuvwxyz" > inputs/02

"$EZLZ" <inputs/02 >compressed/02
"$UNEZLZ" <compressed/02 >outputs/02

cmp inputs/02 outputs/02
