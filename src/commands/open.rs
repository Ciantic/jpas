use clap::Clap;
use derive_more::From;
use std::{
    fs,
    io::{ErrorKind, Read},
    path::PathBuf,
};

use crate::{
    json::{json_add_file, json_decrypt_prop},
    Error,
};

#[derive(Clap, Debug)]
pub struct OpenOpts {
    pub file: Option<PathBuf>,
}

pub fn open(opts: OpenOpts, stdin: &mut dyn Read) -> Result<serde_json::Value, Error> {
    let mut json = match opts.file {
        // User provided JSON file path as a cli argument
        Some(filepath) => {
            let contents = std::fs::read_to_string(&filepath)?;
            let mut json: serde_json::Value = serde_json::from_str(&contents)?;
            json_add_file(&mut json, filepath)?;
            json
        }

        // User is expected to provide JSON through stdin
        None => {
            let mut buffer = String::new();
            stdin.read_to_string(&mut buffer)?;
            let json: serde_json::Value = serde_json::from_str(&buffer)?;
            json
        }
    };
    json_decrypt_prop(&mut json, "secrets")?;
    Ok(json)
}

#[cfg(test)]
mod tests {
    use normpath::PathExt;
    use once_cell::sync::Lazy;
    use serde_json::json;
    use std::path::PathBuf;

    use crate::tests::test_init_gpghome;

    use super::{open, OpenOpts};

    /// `$ jpas save "./tests/basic/Example.ssh.json"`
    #[test]
    fn test_open_from_file() {
        test_init_gpghome();
        let examplefile: PathBuf = PathBuf::from("./tests/basic/Example.ssh.json");
        let json = open(
            OpenOpts {
                file: Some(examplefile.clone()),
            },
            &mut "".as_bytes(),
        )
        .unwrap();

        assert_eq!(
            json,
            json!({
                "type": "ssh",
                "use public key": "00:11:22:..",
                "server": "192.168.1.101",
                "known hosts": "something",
                "secrets": {
                  "password": "swordfish"
                },
                "$file": examplefile
            })
        );
    }

    /// `$ cat ./tests/basic/Example.ssh.json | jpas open`
    #[test]
    fn test_open_from_stdin() {
        test_init_gpghome();
        let examplefile: PathBuf = PathBuf::from("./tests/basic/Example.ssh.json");
        let example_contents = std::fs::read_to_string(examplefile).unwrap();
        let json = open(OpenOpts { file: None }, &mut example_contents.as_bytes()).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "ssh",
                "use public key": "00:11:22:..",
                "server": "192.168.1.101",
                "known hosts": "something",
                "secrets": {
                  "password": "swordfish"
                }
            })
        );
    }
}
