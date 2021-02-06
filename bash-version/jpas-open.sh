#!/bin/bash
set -o pipefail

# Get JSON from file if given, otherwise expects the JSON to come from stdin
file="$1"
if [ -n "$file" ]; then
    input_json=$(<"$file")
else
    input_json=$(</dev/stdin)
fi

# Decrypt the secrets property
secrets=$(jq --raw-output '.secrets' <<< "$input_json" | gpg -q --decrypt 2>/dev/null)
if [ $? -ne 0 ] ; then
    echo -e "\e[01;31mUnable to decrypt secrets\e[0m" >&2
    exit 2
fi

# Add decrypted `secrets` property and `$file` property with the input file if any
jq --argjson secrets "$secrets" --arg file "$file" '.secrets = $secrets | if ($file|length > 0) then ."$file" = $file else . end' <<< "$input_json"
if [ $? -ne 0 ] ; then
    echo -e "\e[01;31mUnable to append secrets\e[0m" >&2
    exit 2
fi