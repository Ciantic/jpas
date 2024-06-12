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

# Get the public keyids used to decrypt the secrets
# 
# Example line:
#   :pubkey enc packet: version 3, algo 18, keyid ABCDEF0123456789
# Parse only the keyid, secrets_fprs contains newline separated keyids
secrets_fprs=$(gpg --list-packets <<< "$secrets" 2>&1 | grep -oP 'pubkey .* keyid \K[0-9A-F]+' | sort | uniq)

# Associative array fields
declare -A fields

fields["secrets"]="$secrets"
fields["secrets_fprs"]=$(jq --raw-input --null-input '[inputs | select(length>0)]' <<< "$secrets_fprs")

# Extract other fields from the input JSON
field_names=(id name type urls username emails notes url_match_rules data)
for field_name in "${field_names[@]}"; do
    fields["$field_name"]=$(jq --raw-output -c ".$field_name // -1" <<< "$input_json")
done

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