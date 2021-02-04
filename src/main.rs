use clap::Clap;
use derive_more::From;
use serde_json::{Map, Value};
use std::{
    io::{stdin, Read, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
use url::Url;

mod gpg;

#[derive(Clap)]
#[clap(
    version = "1.0",
    author = "Jari O. O. Pennanen <jari.pennanen@gmail.com>"
)]
struct Opts {
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,

    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    /// Open password file entry and outputs to stdout
    Open(OpenOpts),

    /// Save password, requires the result of open in stdin
    Save(SaveOpts),
}

#[derive(Clap, Debug)]
struct SaveOpts {
    file: Option<PathBuf>,
}

#[derive(Clap, Debug)]
struct OpenOpts {
    input: Option<String>,
}

#[derive(From, Debug)]
pub enum Error {
    // Yaml(serde_yaml::Error),
    Json(serde_json::Error),
    Io(std::io::Error),
    FileMissing,
    SaveInput,
    RequiresJsonObject,
    SecretsPropertyMissing,
    SecretsMustBeString,
    SecretsAlreadyDecrypted,
    Gpg(gpg::Error),
}

fn json_add_filename(json: &mut serde_json::Value, file: PathBuf) -> Result<(), Error> {
    match json {
        serde_json::Value::Object(value) => {
            value.insert("$file".into(), file.to_string_lossy().into());
            Ok(())
        }
        _ => Err(Error::RequiresJsonObject),
    }
}

fn json_get_filename(json: &serde_json::Value) -> Result<PathBuf, Error> {
    match &json["$file"] {
        serde_json::Value::String(filename) => Ok(PathBuf::from(filename)),
        serde_json::Value::Null => Err(Error::RequiresJsonObject),
        _ => Err(Error::FileMissing),
    }
}

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

fn json_encrypt_prop(json: &mut serde_json::Value, prop: &str) -> Result<(), Error> {
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

// fn json_decrypt_secrets(value: &mut Map<String, Value>) -> Result<(), Error> {
//     match value.get("secrets") {
//         Some(serde_json::Value::String(gpg_secrets)) => {
//             let decrypted = gpg::decrypt(gpg_secrets)?;
//             let decrypted_json = serde_json::from_str(&decrypted)?;
//             value.insert("secrets".into(), decrypted_json);
//             Ok(())
//         }
//         Some(serde_json::Value::Object(_)) => Err(Error::SecretsAlreadyDecrypted),
//         Some(_) => Err(Error::SecretsMustBeString),
//         None => Err(Error::SecretsPropertyMissing),
//     }
// }

// fn json_encrypt_secrets(value: &mut Map<String, Value>) -> Result<(), Error> {
//     match value.get("secrets") {
//         Some(v) => {
//             value.insert("secrets".into(), serde_json::Value::String(v.to_string()));
//             Ok(())
//         }
//         None => Err(Error::SecretsPropertyMissing),
//     }
// }

fn main() -> Result<(), Error> {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Open(opts) => {
            let (mut json, file) = match opts.input {
                // User provided file in a cli argument
                Some(filename) => {
                    let filepath = PathBuf::from(filename);
                    let contents = std::fs::read_to_string(&filepath)?;
                    let json: serde_json::Value = serde_json::from_str(&contents)?;
                    (json, filepath)
                }
                // User most likely wants to provide JSON by piping with stdin
                None => {
                    let mut buffer = String::new();
                    std::io::stdin().read_to_string(&mut buffer)?;
                    let json: serde_json::Value = serde_json::from_str(&buffer)?;
                    let file = json_get_filename(&json)?;
                    (json, file)
                }
            };
            json_add_filename(&mut json, file)?;
            json_decrypt_prop(&mut json, "secrets")?;
            println!("{}", serde_json::to_string_pretty(&json)?);
            Ok(())
        }
        SubCommand::Save(opts) => {
            // Read JSON from stdin
            let mut buffer = String::new();
            std::io::stdin().read_to_string(&mut buffer)?;
            let mut json = serde_json::from_str(&buffer)?;

            // Get save file from cli option, or $file property in JSON
            let file: PathBuf = match opts.file {
                Some(v) => v,
                None => json_get_filename(&json)?,
            };

            // Encrypt and save the JSON
            json_encrypt_prop(&mut json, "secrets")?;
            std::fs::write(file, serde_json::to_string_pretty(&json)?)?;
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    pub fn test_open() {}
}
