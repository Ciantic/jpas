# jpas

Work in progress.

Idea is to parse JSON, with a property called secrets, that is PGP encrypted JSON value:

```json
{
    "type": "website",
    "url": "https://example.com",
    "secrets": "-----BEGIN PGP MESSAGE-----..."
}
```

When decrypted it looks like this:

```json
{
    "type": "website",
    "url": "https://example.com",
    "secrets": {
        "password": "swordfish"
    }
}
```

Genrally jpas is not very useful at the moment, you can do without it pretty easily with just:

```bash
cat Example.website.json | jq -r '.secrets' | gpg -q --decrypt 2>/dev/null | jq -r '.password'
# Gets the password swordfish
```

## Examples

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

## TODO

JSON Schema per entry:

-   Website
-   SSH/SFTP-site

Generally `jpas` could be done rather trivially with existing tools, e.g. `jq` and `gpg`, and piping, so perhaps this tool is not very useful in own.

## More secure JSON editor

It would be nice to make cli based JSON editor that takes input from stdin and outputs the finald result to stdout. Then I could use it like `vipe`, but without intermediate files.

```bash
jpas open "Some site.json" | somejsoneditor | jpas save
```
