#!/bin/bash
export GNUPGHOME=$(mktemp -d)
gpg --batch --passphrase '' --quick-generate-key "John Doe <john@example.com>" future-default default never
gpg --output test-private.keys --armor --export-secret-keys 