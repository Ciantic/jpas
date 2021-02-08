use std::{fs, path::PathBuf};

use crate::{ConfigFile, Error};

pub fn init() -> Result<(), Error> {
    let config_file = PathBuf::from("jpas.json");
    if config_file.exists() {
        Err(Error::InitAlreadyDone)
    } else {
        let contents: String = serde_json::to_string_pretty(&ConfigFile {
            save_other_gpg_recipients: None,
        })?;
        fs::write(config_file, contents.as_bytes())?;
        Ok(())
    }
}
