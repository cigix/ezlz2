#!/bin/sh

if [ -z "$EZLZ" -o -z "$UNEZLZ" ]
then
    echo "This script requires environment variables EZLZ and UNEZLZ be set"
    exit 1
fi

# Ratio test: is the file actually compressed

mkdir -p inputs compressed outputs
echo "be you, be proud of you because you can be do what we want to do" > inputs/06

"$EZLZ" <inputs/06 >compressed/06

input_size=$(du -b inputs/06 | cut -f1)
compressed_size=$(du -b compressed/06 | cut -f1)
echo "$compressed_size < $input_size"
[ "$compressed_size" -lt "$input_size" ]
