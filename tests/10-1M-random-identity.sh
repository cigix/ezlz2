#!/bin/sh

if [ -z "$EZLZ" -o -z "$UNEZLZ" ]
then
    echo "This script requires environment variables EZLZ and UNEZLZ be set"
    exit 1
fi

# Identity test: does decompression bring back the initial file

mkdir -p inputs compressed outputs
dd if=/dev/urandom of=inputs/10 bs=1M count=1 2>/dev/null

"$EZLZ" <inputs/10 >compressed/10
"$UNEZLZ" <compressed/10 >outputs/10

cmp inputs/10 outputs/10
