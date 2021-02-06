#!/bin/bash
set -o pipefail
me=$(basename "$0")
password=$(./jpas-open.sh "../tests/Example.ssh.json" | jq -r ".password")

if [[ "$password" == "swordfish" ]]; then
    echo -e "\e[0;32mOk $me\e[0m"
else
    echo -e "\e[01;31mFailed $me\e[0m" >&2
fi