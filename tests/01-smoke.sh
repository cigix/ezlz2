#!/bin/sh

if [ -z "$EZLZ" -o -z "$UNEZLZ" ]
then
    echo "This script requires environment variables EZLZ and UNEZLZ be set"
    exit 1
fi

# Smoke test: run with no input or output

"$EZLZ" </dev/null >/dev/null
if [ $? -ne 0 ]
then
    echo "Could not run $EZLZ with empty input"
    exit 1
fi
"$UNEZLZ" </dev/null >/dev/null
if [ $? -ne 0 ]
then
    echo "Could not run $UNEZLZ with empty input"
    exit 1
fi
