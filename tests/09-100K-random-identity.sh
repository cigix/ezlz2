#!/bin/sh

if [ -z "$EZLZ" -o -z "$UNEZLZ" ]
then
    echo "This script requires environment variables EZLZ and UNEZLZ be set"
    exit 1
fi

# Identity test: does decompression bring back the initial file

mkdir -p inputs compressed outputs
dd if=/dev/urandom of=inputs/09 bs=10K count=10 2>/dev/null

"$EZLZ" <inputs/09 >compressed/09
"$UNEZLZ" <compressed/09 >outputs/09

cmp inputs/09 outputs/09
