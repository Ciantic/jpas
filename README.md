# jpas

Examples

```bash

# Create or update as one shot
jpas open "Google.website.json" | jq '.type=website | .url=https://google.com | .secrets.password=swordfish' | jpas save
jpas open "My Server.ssh.json" | jq '.type=ssh | .server=192.168.8.150' | jpas save
jpas open "Some Weird App.other.json" | jq '.url=https://example.com | .desc="This is a very weird application" | .secrets.password=swordfish' | jpas save

# Edit with your editor and save (requires moreutils with vipe),
# apparently vipe does a temp file which might not be secure.
jpas open "Some site.json" | vipe | jpas save

# Find by url example.com, get password, move to clipboard
find -name "*.website.json" | ... # jpas open | jq ".secrets.password" | jpas clip

```

## Tests

Tests should generate new `GNUPGHOME` directory under `./tests/gpg/.gnupghome/`, if
something fails on creating it, the tests will fail too. Delete that directory
if tests ceases to function for some reason.
