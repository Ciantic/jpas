#!/bin/bash
export GNUPGHOME=$(mktemp -d)
gpg --batch --passphrase '' --import ./test-private.keys

# Example:
echo '{ "password" : "swordfish" }' | gpg --default-recipient-self --armor --sign --encrypt | sed -z 's/\n/\\n/g' | clip.exe


bash