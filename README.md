# jpas

Work in progress. This has same ideas as the passwordstore.org, but I wanted to
build on limitation that signing and decrypting requires touch on YubiKey. This
means I can't GPG encrypt whole entry, and parts of the entry must be indexable.

jpas open and save commands [are also available as bash
scripts](./bash-version/) in the `bash-version` directory. Currently
the Rust binary replicates the bash script behavior, and not very useful.

## Behavior

Saved (encrypted) entries looks like this:

```json
{
    "type": "website",
    "url": "https://example.com",
    "secrets": "-----BEGIN PGP MESSAGE-----..."
}
```

When opened (decrypted) it looks like this:

```json
{
    "type": "website",
    "url": "https://example.com",
    "secrets": {
        "password": "swordfish"
    }
}
```

## Examples

### Saving an entry

```bash
# Save to entry given as argument

echo '{ "url" : "https://example.com", "secrets" : { "password" : "swordfish"} }' | jpas save YourFile.json

# or similarily take the save to file as JSON property $file

echo '{ "url" : "https://example.com", "secrets" : { "password" : "swordfish"}, "$file" : "YourFile.json" }' | jpas save
```

Above creates `YourFile.json` with contents:

```json
{ "url": "https://example.com", "secrets": "-----BEGIN PGP MESSAGE-----..." }
```

### Opening an entry

```bash
# Open entry by argument

jpas open YourFile.json

# or open entry given in stdin

cat YourFile.json | jpas open
```

### Other examples

```bash
# JQ tricks
jpas open "Google.website.json" | jq '.type=website | .url=https://google.com | .secrets.password=swordfish' | jpas save

jpas open "My Server.ssh.json" | jq '.type=ssh | .server=192.168.8.150' | jpas save

jpas open "Some Weird App.other.json" | jq '.url=https://example.com | .desc="This is a very weird application" | .secrets.password=swordfish' | jpas save

# Open in an editor
#
# Edit with your editor and save (requires moreutils with vipe),
# apparently vipe does a temp file which might not be secure.
jpas open "Some site.json" | vipe | jpas save
```

## Tests

Tests should generate new `GNUPGHOME` directory under `./tests/gpg/.gnupghome/`,
if something fails on creating it, the tests will fail too. Delete that
directory if tests ceases to function for some reason.

## Todo

-   Some sort of JSON editor with ability to work between pipe without temporary
    files, e.g.

    `jpas-open Entry.json | somejsoneditor | jpas-save`.

    Creating editor is outside the scope of these scripts, as it would require a
    lot interaction and Rust is a better tool for that.

-   Chrome extension which sends the password for a site. It builds index of
    `.website.json` URLs using call to executable with [native
    messaging](https://developer.chrome.com/extensions/nativeMessaging#native-messaging-host).

## Ideas & guidelines behind

-   Work with YubiKey touch to decrypt, meaning the decrypting is _slow_ and
    requires physical touch on YubiKey. Purpose of this limitation is to make
    stealing all passwords burdensome, as it would require touching YubiKey for
    all entries. [^filippo]
-   Whole entry can't be encrypted, e.g. if you want to index all website URLs
    then the URL properties must not be GPG encrypted. Currently the idea is to
    encrypt only the credentials, e.g. passwords etc.
-   Never store decrypted entry to the file system (as temporary files or
    otherwise).

[^filippo]: Read about Filippo Valsorda's ["Touch to operate Password-store with YubiKey 4"](https://blog.filippo.io/touch-to-operate-password-store-yubikey-4/)
