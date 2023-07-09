#!/usr/bin/env bash

if [ -z "$1" ]; then
    echo "Usage: ${BASH_SOURCE:-$0} <name>"
    exit 1
fi

path=`/usr/bin/dirname ${BASH_SOURCE:-$0}`

/usr/bin/cp -r "$path/.template" "$path/$1"
/usr/bin/sed -i "s/\[template\-name\]/$1/" "$path/$1/Cargo.toml"
echo "Created new folder: $path/$1"
