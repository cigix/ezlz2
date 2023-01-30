#!/bin/sh

if [ -z "$EZLZ" -o -z "$UNEZLZ" ]
then
    echo "This script requires environment variables EZLZ and UNEZLZ be set"
    exit 1
fi

# Identity test: does decompression bring back the initial file

mkdir -p inputs compressed outputs
dd if=/dev/urandom of=inputs/07 bs=1K count=1 2>/dev/null

"$EZLZ" <inputs/07 >compressed/07
"$UNEZLZ" <compressed/07 >outputs/07

cmp inputs/07 outputs/07
