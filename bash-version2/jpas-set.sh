#!/bin/bash

set -e
set -o pipefail

# Trap error and print the error message
trap 'echo -e "\e[01;31mError: $BASH_COMMAND failed at line $LINENO\e[0m"' ERR

database="jpas.sqlite"

input_json=$(</dev/stdin)

# If secrets does not start "-----BEGIN PGP MESSAGE-----", then it needs to be encrypted
secrets=$(jq -c '.secrets' <<< "$input_json")
if [[ ! "$secrets" =~ ^-----BEGIN\ PGP\ MESSAGE----- ]]; then
    secrets=$(echo "$secrets" | gpg --default-recipient-self --armor --sign --encrypt 2>/dev/null)
    if [ $? -ne 0 ] ; then
        echo -e "\e[01;31mUnable to encrypt secrets\e[0m" >&2
        exit 2
    fi
fi

# Associative array fields
declare -A fields

fields["secrets"]="$secrets"
fields["id"]=$(jq --raw-output '.id // -1' <<< "$input_json")
fields["name"]=$(jq --raw-output '.name // -1' <<< "$input_json")
fields["type"]=$(jq --raw-output '.type // -1' <<< "$input_json")
fields["urls"]=$(jq --raw-output -c '.urls // -1' <<< "$input_json")
fields["username"]=$(jq --raw-output '.username // -1' <<< "$input_json")
fields["emails"]=$(jq --raw-output -c '.emails // -1' <<< "$input_json")
fields["notes"]=$(jq --raw-output '.notes // -1' <<< "$input_json")
fields["url_match_rules"]=$(jq --raw-output '.url_match_rules // -1' <<< "$input_json")
fields["data"]=$(jq --raw-output '.data // -1' <<< "$input_json")

SQL_FIELDS=()
SQL_VALUES=()
SQL_UPSERTS=()

# Unfortunately sqlite3 cli has no prepared statement support. Following SQL may
# fail if the input has singe quotes. However this should be used by trusted
# users only anyway.
for key in "${!fields[@]}"; do
    if [ "${fields[$key]}" != "-1" ]; then
        SQL_FIELDS+=("$key")
        SQL_VALUES+=("'${fields[$key]}'")
        SQL_UPSERTS+=("$key = '${fields[$key]}'")
    fi
done

# ON CONFLICT clause:
# CONFLICT - overwrites existing item with same name or id
# CONFLICT(id) - overwrites existing item with same id

SQL="INSERT INTO jpas ($(IFS=,; echo "${SQL_FIELDS[*]}")) VALUES ($(IFS=,; echo "${SQL_VALUES[*]}")) ON CONFLICT DO UPDATE SET $(IFS=,; echo "${SQL_UPSERTS[*]}");"

sqlite3 "$database" "$SQL"