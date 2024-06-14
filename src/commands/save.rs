use clap::Parser;

use crate::json::{json_encrypt_prop, json_get_file, json_remove_file};
use crate::Error;
use std::{io::Read, path::PathBuf};

#[derive(Parser, Debug)]
pub struct SaveOpts {
    pub file: Option<PathBuf>,
}

pub fn save(opts: SaveOpts, stdin: &mut dyn Read) -> Result<(), Error> {
    // Read JSON from stdin
    let mut buffer = String::new();
    stdin.read_to_string(&mut buffer)?;
    let mut json = serde_json::from_str(&buffer)?;

    // Get save filepath from cli option, or $file property in JSON
    let file: PathBuf = match opts.file {
        Some(v) => v,
        None => json_get_file(&json)?,
    };

    // Encrypt and save the JSON
    let _ = json_remove_file(&mut json);
    json_encrypt_prop(&mut json, "secrets")?;
    std::fs::write(file, serde_json::to_string_pretty(&json)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use std::path::PathBuf;

    use crate::tests::test_init_gpghome;

    use super::{save, SaveOpts};

    /// `$ jpas open "./tests/basic/Example.ssh.json" | jpas save`
    #[test]
    fn test_save_from_stdin() {
        test_init_gpghome();
        // Output file, remove for each test time
        let outputfile = PathBuf::from("./tests/basic/test_save_from_stdin.temp.ssh.json");
        let _ = std::fs::remove_file(&outputfile);

        // Save to $file
        let examplejson = serde_json::to_string_pretty(&json!({
            "type": "ssh",
            "use public key": "00:11:22:..",
            "server": "192.168.1.101",
            "known hosts": "something",
            "secrets": {
              "password": "swordfish"
            },
            "$file": outputfile
        }))
        .unwrap();

        save(SaveOpts { file: None }, &mut examplejson.as_bytes()).unwrap();

        let json: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(outputfile).unwrap()).unwrap();

        assert_eq!(
            json["secrets"]
                .as_str()
                .unwrap()
                .starts_with("-----BEGIN PGP MESSAGE-----"),
            true
        );

        assert_eq!(
            json,
            json!({
                "type": "ssh",
                "use public key": "00:11:22:..",
                "server": "192.168.1.101",
                "known hosts": "something",
                "secrets": json["secrets"],
            })
        )
    }

    /// `$ cat "./tests/basic/Example.ssh.json" | jpas save "./tests/basic/test_save_to_file.temp.ssh.json"`
    #[test]
    fn test_save_to_file() {
        test_init_gpghome();
        // Output file, remove for each test time
        let outputfile = PathBuf::from("./tests/basic/test_save_to_file.temp.ssh.json");
        let _ = std::fs::remove_file(&outputfile);

        // Save to $file
        let examplejson = serde_json::to_string_pretty(&json!({
            "type": "ssh",
            "use public key": "00:11:22:..",
            "server": "192.168.1.101",
            "known hosts": "something",
            "secrets": {
              "password": "swordfish"
            },
        }))
        .unwrap();

        save(
            SaveOpts {
                file: Some(outputfile.clone()),
            },
            &mut examplejson.as_bytes(),
        )
        .unwrap();

        let json: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&outputfile).unwrap()).unwrap();

        assert_eq!(
            json["secrets"]
                .as_str()
                .unwrap()
                .starts_with("-----BEGIN PGP MESSAGE-----"),
            true
        );

        assert_eq!(
            json,
            json!({
                "type": "ssh",
                "use public key": "00:11:22:..",
                "server": "192.168.1.101",
                "known hosts": "something",
                "secrets": json["secrets"],
            })
        )
    }
}
