#!/bin/bash
export GNUPGHOME=$(mktemp -d)
if [[ "$OSTYPE" == "cygwin" ]]; then
    export GNUPGHOME=$(cygpath -w $GNUPGHOME)
fi
pushd "$(dirname "$0")"

gpg --batch --passphrase '' --import ./test-private.keys

# Example:
echo '{ "password" : "swordfish" }' | gpg --default-recipient-self --armor --sign --encrypt | sed -z 's/\n/\\n/g' | clip.exe

popd

bash