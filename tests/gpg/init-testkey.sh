#!/bin/bash
export GNUPGHOME=$(mktemp -d)
if [[ "$OSTYPE" == "cygwin" ]]; then
    export GNUPGHOME=$(cygpath -w $GNUPGHOME)
fi
pushd "$(dirname "$0")"

gpg --batch --passphrase '' --quick-generate-key "John Doe <john@example.com>" future-default default never
gpg --output test-private.keys --armor --export-secret-keys 

popd