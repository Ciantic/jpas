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
    file: Option<String>,
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
    SaveFileMissing,
    SaveInput,
    DecryptRequiresJsonObject,
    SecretsPropertyMissing,
    SecretsMustBeString,
    SecretsAlreadyDecrypted,
    Gpg(gpg::Error),
}

fn add_filename(value: &mut Map<String, Value>, file: PathBuf) {
    value.insert("$file".into(), file.to_string_lossy().into());
}

fn decrypt_secrets(value: &mut Map<String, Value>) -> Result<(), Error> {
    match value.get("secrets") {
        Some(serde_json::Value::String(gpg_secrets)) => {
            let decrypted = gpg::decrypt(gpg_secrets)?;
            let decrypted_json = serde_json::from_str(&decrypted)?;
            value.insert("secrets".into(), decrypted_json);
            Ok(())
        }
        Some(serde_json::Value::Object(_)) => Err(Error::SecretsAlreadyDecrypted),
        Some(_) => Err(Error::SecretsMustBeString),
        None => Err(Error::SecretsPropertyMissing),
    }
}

fn encrypt_secrets(value: &mut Map<String, Value>) -> Result<(), Error> {
    todo!()
}

fn main() -> Result<(), Error> {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Open(values) => {
            let mut file = None;
            let input = match values.input {
                Some(filename) => {
                    let filepath = PathBuf::from(filename);
                    let contents = std::fs::read_to_string(&filepath)?;
                    file = Some(filepath);
                    contents
                }
                None => {
                    let mut buffer = String::new();
                    std::io::stdin().read_to_string(&mut buffer)?;
                    buffer
                }
            };

            if let serde_json::Value::Object(mut value) = serde_json::from_str(&input)? {
                if let Some(file) = file {
                    add_filename(&mut value, file);
                }
                decrypt_secrets(&mut value)?;
                println!("{}", serde_json::to_string_pretty(&value)?);
                Ok(())
            } else {
                Err(Error::DecryptRequiresJsonObject)
            }
        }
        SubCommand::Save(values) => {
            let file = values.file;
            let mut buffer = String::new();
            std::io::stdin().read_to_string(&mut buffer)?;

            if let serde_json::Value::Object(mut values) = serde_json::from_str(&buffer)? {
                let file_in_json = values.get("$file").map(|v| v.as_str().unwrap_or("").into());
                if let Some(file) = file.or(file_in_json) {
                    let filepath = PathBuf::from(file);
                    let _ = values.remove("$file");
                    encrypt_secrets(&mut values)?;
                    let json = serde_json::to_string_pretty(&serde_json::Value::Object(values))?;
                    std::fs::write(filepath, json)?;
                    Ok(())
                } else {
                    Err(Error::SaveFileMissing)
                }
            } else {
                Err(Error::SaveInput)
            }
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    pub fn test_open() {}
}
