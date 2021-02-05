# pass

Examples

```bash

# Create or update as one shot
pass open "Google.website.json" | jq '.type=website | .url=https://google.com | .secrets.password=swordfish' | pass save
pass open "My Server.ssh.json" | jq '.type=ssh | .server=192.168.8.150' | pass save
pass open "Some Weird App.other.json" | jq '.url=https://example.com | .desc="This is a very weird application" | .secrets.password=swordfish' | pass save

# Edit with your editor and save (requires moreutils with vipe),
# apparently vipe does a temp file which might not be secure.
pass open "Some site.json" | vipe | pass save

# Find by url example.com, get password, move to clipboard
find -name "*.website.json" | ... # pass open | jq ".secrets.password" | pass clip

```

## Tests

Tests should generate new `GNUPGHOME` directory under `tests/gpg/.gnupghome`, if
something fails on creating it, the tests will fail too. Delete that directory
if tests ceases to function for some reason.
