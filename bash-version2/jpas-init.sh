#!/bin/bash

set -e
set -o pipefail

database="jpas.sqlite"

if [ -f "$database" ]; then
    echo -e "\e[01;31mDatabase already exists\e[0m" >&2
    exit 1
fi

# Pass init.sql to sqlite3 to create the database
sqlite3 "$database" < init.sql