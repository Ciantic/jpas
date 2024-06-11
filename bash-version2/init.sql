-- SQLITE3 file

CREATE TABLE jpas (
    id INTEGER PRIMARY KEY,

    name TEXT NOT NULL UNIQUE,

    type TEXT NOT NULL DEFAULT 'website',
    urls TEXT NOT NULL DEFAULT '[]' CHECK (json_valid(urls) AND json_type(urls) = 'array'),
    username TEXT NOT NULL DEFAULT '',

    -- emails associated with the account
    emails TEXT NOT NULL DEFAULT '[]' CHECK (json_valid(emails) AND json_type(emails) = 'array'),
    notes TEXT NOT NULL DEFAULT '',

    -- rules for URL matching, e.g. {"https://example.com": "exact"}
    url_match_rules TEXT NOT NULL DEFAULT '{}' CHECK (json_valid(url_match_rules) AND json_type(url_match_rules) = 'object'),

    -- json data, can be anything
    data TEXT NOT NULL DEFAULT '{}' CHECK (json_valid(data) AND json_type(data, '$.secrets') IS NULL),

    -- encrypted secrets (json) in PGP format or empty string
    secrets TEXT NOT NULL CHECK (secrets LIKE '-----BEGIN PGP MESSAGE-----%' OR secrets = ''),

    -- finger prints of the public keys used to decrypt the secrets
    secrets_fprs TEXT NOT NULL DEFAULT '[]' CHECK (json_type(secrets_fprs) = 'array'),

    -- unix timestamps as integers
    created_at INT NOT NULL DEFAULT (CAST(strftime('%s', 'now') as INT)),
    updated_at INT NOT NULL DEFAULT (CAST(strftime('%s', 'now') as INT))
) STRICT;


-- INSERT INTO jpas (name, username, urls, secrets, secrets_fprs) VALUES ('Example.Com', 'johndoe', '["https://example.com"]', '-----BEGIN PGP MESSAGE-----', '["fingerprint1", "fingerprint2"]');

-- INSERT INTO jpas (name, username, urls, secrets, secrets_fprs) VALUES ('The Foo Site 1', 'johnd', '["https://foo.com"]', '-----BEGIN PGP MESSAGE-----', '["fingerprint1", "fingerprint2"]');

-- INSERT INTO jpas (name, username, urls, secrets, secrets_fprs) VALUES ('The Foo Site', 'johnd', '["https://foo.com", "https://example.com"]', '-----BEGIN PGP MESSAGE-----', '["fingerprint1", "fingerprint2"]');

