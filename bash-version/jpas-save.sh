#!/bin/bash

# Save the password entry
# 
# 1. Gets the JSON from stdin
# 2. Encrypts the `secrets` JSON property with GPG
# 3. Saves full JSON to given file
#
# Examples: 
#
# $ echo '{ "url" : "https://example.com", "secrets" : { "password" : "swordfish"} }' | ./jpas-save.sh YourFile.json
# $ echo '{ "url" : "https://example.com", "secrets" : { "password" : "swordfish"}, "$file" : "YourFile.json" }' | ./jpas-save.sh
#
# Above commands are equivalent, and ends up creating `YourFile.json` with contents:
# 
# { "url" : "https://example.com", "secrets": "-----BEGIN PGP MESSAGE-----..." }

set -o pipefail

# Input is expected from stdin
stdin=$(</dev/stdin)

# Get the save to file from first program argument, or JSON `$file` property in the input 
save_to="$1"
if [ -z "$save_to" ]; then
    save_to=$(jq --exit-status --raw-output '."$file"' <<< "$stdin")
    if [ $? -ne 0 ] ; then
        echo -e "\e[01;31mError: Save to file is missing, please give save file as first argument\e[0m" >&2
        exit 2
    fi
fi

# Encrypt the `secrets` JSON property with GPG
secrets=$(jq --exit-status '.secrets' <<< "$stdin" | gpg --default-recipient-self --armor --sign --encrypt 2>/dev/null)
if [ $? -ne 0 ] ; then
    echo -e "\e[01;31mUnable to encrypt secrets\e[0m" >&2
    exit 2
fi

# 1. Replace secrets property with GPG armored secrets string
# 2. Remove `$file` property if it exists
jq --arg secrets "$secrets" '.secrets = $secrets | del(."$file")' <<< "$stdin" > "$save_to"