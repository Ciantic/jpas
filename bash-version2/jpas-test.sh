#!/bin/bash

set -e
set -o pipefail

# Trap error and print the error message
trap 'echo -e "\e[01;31m❌ Error: $BASH_COMMAND failed at line $LINENO\e[0m"' ERR

rm jpas.sqlite && ./jpas-init.sh
echo '{ "secrets" : "foo", "name": "foo", "urls": ["foo", "faa"] }' | ./jpas-set.sh
./jpas-get.sh --name foo
./jpas-get.sh --url foo
./jpas-get.sh --url foo | ./jpas-decrypt.sh

echo "✅ All tests passed"