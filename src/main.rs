use clap::Clap;
use derive_more::From;
use std::{io::Read, path::PathBuf};

mod commands;
mod gpg;
mod json;
use commands::{
    init::init,
    open::{open, OpenOpts},
    query::{query, QueryOpts},
    save::SaveOpts,
};
use json::*;

#[derive(Clap, Debug)]
#[clap(version = "0.1", author = "Jari O. O. Pennanen <ciantic@oksidi.com>")]
struct Opts {
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,

    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap, Debug)]
enum SubCommand {
    /// Open entry and outputs to stdout
    Open(OpenOpts),

    /// Save entry to a file
    Save(SaveOpts),

    /// Query directory for specific entry
    Query(QueryOpts),

    /// Init
    Init,
}

#[derive(From, Debug)]
pub enum Error {
    // Yaml(serde_yaml::Error),
    Json(serde_json::Error),
    Io(std::io::Error),
    FileMissing,
    FileMustBeString,
    RequiresJsonObject,
    SecretsPropertyMissing,
    SecretsMustBeString,
    SecretsAlreadyDecrypted,
    Gpg(gpg::Error),
    InitAlreadyDone,
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

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ConfigFile {
    save_other_gpg_recipients: Option<Vec<String>>,
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
        SubCommand::Query(opts) => query(opts),
        SubCommand::Init => init(),
    }
}

#[cfg(test)]
mod tests {
    use normpath::PathExt;
    use once_cell::sync::Lazy;
    use std::path::PathBuf;

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
}
