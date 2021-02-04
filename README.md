# pass

Examples

```bash

# Create or update as one shot
pass decrypt "Google.yaml" | jq '.type=website | .url=https://google.com | .secrets.password=swordfish' | pass save
pass decrypt "My Server.yaml" | jq '.type=ssh | .server=192.168.8.150' | pass save
pass decrypt "Some Weird App.yaml" | jq '.url=https://example.com | .desc="This is a very weird application" | .secrets.password=swordfish' | pass save
pass decrypt "Some site.yaml" | jq ".url=" | pass save

# Open an editor
pass "Site.yaml"

# Find example.com, get password, move to clipboard
# Note: forall never decrypts by default (because it's not really feasible if you use YubiKey's touch feature)

find -name "*.website.yaml" | ... # pass decrypt | jq ".secrets.password" | pass clip

```

## Tests

Tests should generate new `GNUPGHOME` directory under `tests/gpg/.gnupghome`, if something fails on creating it, the tests will fail too. Delete that directory if tests ceases to function for some reason.
