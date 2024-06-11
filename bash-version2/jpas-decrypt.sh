#!/bin/bash

set -e 
set -o pipefail

# Trap error and print the error message
trap 'echo -e "\e[01;31mError: $BASH_COMMAND failed at line $LINENO\e[0m"' ERR

input_json=$(</dev/stdin)

secrets=$(jq --raw-output '.secrets' <<< "$input_json" | gpg -q --decrypt 2>/dev/null)

jq --argjson secrets "$secrets" '.secrets = $secrets' <<< "$input_json"