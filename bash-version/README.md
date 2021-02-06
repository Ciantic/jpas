# jpas open and save as bash scripts

Work in progress. This has same ideas as the passwordstore.org, but I wanted to
build on limitation that signing and decrypting requires touch on YubiKey. This
means I can't GPG encrypt whole entry, and parts of the entry must be indexable.

Naturally one could symmetrically encrypt the whole store additionally, but it's
outside of the scope of this project. However it wouldn't add whole lot of
security if your file system is comporomised as the symmetric key would then
also be compromised.

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

## TODO

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
-   Bash scripts with no functions or other shenanigans, they are readable from
    top to down. Generally I don't like bash scripts, but since these scripts are
    supposed to be simple they are easier to understand than binaries with plenty
    of dependencies.
-   Bash scripts should not depend on other bash scripts.
-   Bash scripts should depend only on few known tools, currently `jq`, `gpg`.
-   I use shellcheck but it's just VSCode's defaults what ever those are.

[^filippo]: Read about Filippo Valsorda's ["Touch to operate Password-store with YubiKey 4"](https://blog.filippo.io/touch-to-operate-password-store-yubikey-4/)
