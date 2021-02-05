use clap::Clap;
use derive_more::From;
use normpath::PathExt;
use serde_json::{Map, Value};
use std::{
    io::{stdin, Read, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

mod gpg;

#[derive(Clap)]
#[clap(version = "1.0", author = "Jari O. O. Pennanen <ciantic@oksidi.com>")]
struct Opts {
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,

    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    /// Open file entry and outputs to stdout
    Open(OpenOpts),

    /// Save file entry
    Save(SaveOpts),
}

#[derive(Clap, Debug)]
struct SaveOpts {
    file: Option<PathBuf>,
}

#[derive(Clap, Debug)]
struct OpenOpts {
    file: Option<PathBuf>,
}

#[derive(From, Debug)]
pub enum Error {
    // Yaml(serde_yaml::Error),
    Json(serde_json::Error),
    Io(std::io::Error),
    FileMissing,
    FileMustBeString,
    SaveInput,
    RequiresJsonObject,
    SecretsPropertyMissing,
    SecretsMustBeString,
    SecretsAlreadyDecrypted,
    Gpg(gpg::Error),
}

/// Add $file property to JSON
fn json_add_file(json: &mut serde_json::Value, file: PathBuf) -> Result<(), Error> {
    match json {
        serde_json::Value::Object(value) => {
            value.insert("$file".into(), file.to_string_lossy().into());
            Ok(())
        }
        _ => Err(Error::RequiresJsonObject),
    }
}

/// Remove the $file property from JSON
fn json_remove_file(json: &mut serde_json::Value) -> Result<(), Error> {
    match json {
        serde_json::Value::Object(value) => {
            let _ = value.remove("$file");
            Ok(())
            /*
            if let serde_json::Value::String(file) =
                value.remove("$file").ok_or(Error::FileMissing)?
            {
                Ok(PathBuf::from(file))
            } else {
                Err(Error::FileMustBeString)
            }
             */
        }
        _ => Err(Error::RequiresJsonObject),
    }
}

/// Get $file property from JSON
fn json_get_file(json: &serde_json::Value) -> Result<PathBuf, Error> {
    match &json["$file"] {
        serde_json::Value::String(filename) => Ok(PathBuf::from(filename)),
        serde_json::Value::Null => Err(Error::FileMissing),
        _ => Err(Error::FileMustBeString),
    }
}

/// Decrypt the GPG encrypted JSON property
fn json_decrypt_prop(json: &mut serde_json::Value, prop: &str) -> Result<(), Error> {
    match json {
        serde_json::Value::Object(value) => match value.get(prop) {
            Some(serde_json::Value::String(gpg_secrets)) => {
                let decrypted = gpg::decrypt(gpg_secrets)?;
                let decrypted_json = serde_json::from_str(&decrypted)?;
                value.insert(prop.into(), decrypted_json);
                Ok(())
            }
            Some(serde_json::Value::Object(_)) => Err(Error::SecretsAlreadyDecrypted),
            Some(_) => Err(Error::SecretsMustBeString),
            None => Err(Error::SecretsPropertyMissing),
        },
        _ => Err(Error::RequiresJsonObject),
    }
}

/// GPG encrypt the property value to string
fn json_encrypt_prop(json: &mut serde_json::Value, prop: &str) -> Result<(), Error> {
    match json {
        serde_json::Value::Object(value) => match value.get(prop) {
            Some(v) => {
                let json_encrypted = gpg::encrypt(&serde_json::to_string(v)?)?;
                value.insert(prop.into(), serde_json::Value::String(json_encrypted));
                Ok(())
            }
            None => Err(Error::SecretsPropertyMissing),
        },
        _ => Err(Error::RequiresJsonObject),
    }
}

fn open(opts: OpenOpts, stdin: &mut dyn Read) -> Result<serde_json::Value, Error> {
    let mut json = match opts.file {
        // User provided JSON file path as a cli argument
        Some(filename) => {
            let filepath = PathBuf::from(filename).normalize()?;
            let contents = std::fs::read_to_string(&filepath)?;
            let mut json: serde_json::Value = serde_json::from_str(&contents)?;
            json_add_file(&mut json, filepath.into())?;
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

fn save(opts: SaveOpts, stdin: &mut dyn Read) -> Result<(), Error> {
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

fn main() -> Result<(), Error> {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Open(opts) => {
            let json = open(opts, &mut std::io::stdin())?;
            println!("{}", serde_json::to_string_pretty(&json)?);
            Ok(())
        }
        SubCommand::Save(opts) => {
            save(opts, &mut std::io::stdin())?;
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use normpath::PathExt;
    use once_cell::sync::Lazy;
    use serde_json::json;
    use std::path::PathBuf;

    use crate::{open, save, OpenOpts, SaveOpts};

    // We only want to run this once, so this is lazy static
    static CREATE_GPGHOME: Lazy<()> = Lazy::new(|| {
        let gnupghome = PathBuf::from("./tests/gpg/.gnupghome");

        // Create GNUPGHOME directory (notice that it fails if it already exists)
        if std::fs::create_dir(&gnupghome).is_ok() {
            std::env::set_var("GNUPGHOME", gnupghome.normalize().unwrap());
            std::process::Command::new("gpg")
                .arg("--batch")
                .arg("--passphrase")
                .arg("")
                .arg("--import")
                .arg(
                    PathBuf::from("./tests/gpg/test-private.keys")
                        .normalize()
                        .unwrap(),
                )
                .status()
                .expect("Failed to import the keys");
        } else {
            std::env::set_var("GNUPGHOME", gnupghome.normalize().unwrap());
        }
    });

    pub fn test_init_gpghome() {
        Lazy::force(&CREATE_GPGHOME);
    }

    /// `$ jpas save "./tests/basic/Example.ssh.json"`
    #[test]
    fn test_open_from_file() {
        test_init_gpghome();
        let examplefile: PathBuf = PathBuf::from("./tests/basic/Example.ssh.json")
            .normalize()
            .unwrap()
            .into();
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
        let examplefile: PathBuf = PathBuf::from("./tests/basic/Example.ssh.json")
            .normalize()
            .unwrap()
            .into();
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
