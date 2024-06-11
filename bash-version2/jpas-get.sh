#!/bin/bash

# Get by --name NAME or --url URL

set -e
set -o pipefail

# Trap error and print the error message
trap 'echo -e "\e[01;31mError: $BASH_COMMAND failed at line $LINENO\e[0m"' ERR

database="jpas.sqlite"

name=""
url=""

# If first argument is --name
if [ "$1" == "--name" ]; then
    name="$2"
fi

if [ "$1" == "--url" ]; then
    url="$2"
fi

sql=""

# Sqlite3 cli has no prepared statement support unfortunately. Following SQL
# building is not injection safe, but this should be used by trusted users only
# anyway. 

if [ -n "$name" ]; then
    sql="SELECT jpas.* FROM jpas WHERE name LIKE '$name%'"
fi

if [ -n "$url" ]; then
    sql="SELECT jpas.* FROM jpas, json_each(jpas.urls) WHERE json_each.value LIKE '$url%'"
fi

if [ -z "$sql" ]; then
    echo -e "\e[01;31mUsage: --url <URL> or --name <NAME>\e[0m" >&2
    exit 2
fi

# echo "$sql"

sqlite3 --json "$database" "$sql" \
    | jq 'map(. + ({ data, secrets_fprs, urls, url_match_rules, emails } | map_values(fromjson)))' \
    | jq '.[0]'

