#!/bin/sh

if [ -z "$EZLZ" -o -z "$UNEZLZ" ]
then
    echo "This script requires environment variables EZLZ and UNEZLZ be set"
    exit 1
fi

# Identity test: does decompression bring back the initial file

mkdir -p inputs compressed outputs
dd if=/dev/urandom of=inputs/08 bs=1K count=10 2>/dev/null

"$EZLZ" <inputs/08 >compressed/08
"$UNEZLZ" <compressed/08 >outputs/08

cmp inputs/08 outputs/08
