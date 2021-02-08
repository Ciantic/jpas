use super::gpg;
use super::Error;
use std::path::PathBuf;

/// Add $file property to JSON
pub fn json_add_file(json: &mut serde_json::Value, file: PathBuf) -> Result<(), Error> {
    match json {
        serde_json::Value::Object(value) => {
            value.insert("$file".into(), file.to_string_lossy().into());
            Ok(())
        }
        _ => Err(Error::RequiresJsonObject),
    }
}

/// Remove the $file property from JSON
pub fn json_remove_file(json: &mut serde_json::Value) -> Result<(), Error> {
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
pub fn json_get_file(json: &serde_json::Value) -> Result<PathBuf, Error> {
    match &json["$file"] {
        serde_json::Value::String(filename) => Ok(PathBuf::from(filename)),
        serde_json::Value::Null => Err(Error::FileMissing),
        _ => Err(Error::FileMustBeString),
    }
}

/// Decrypt the GPG encrypted JSON property
pub fn json_decrypt_prop(json: &mut serde_json::Value, prop: &str) -> Result<(), Error> {
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
pub fn json_encrypt_prop(json: &mut serde_json::Value, prop: &str) -> Result<(), Error> {
    match json {
        serde_json::Value::Object(value) => match value.get(prop) {
            // TODO: Should I prevent double encrypting by failing with String values?
            // Some(serde_json::Value::String(_)) => Err(Error::RequiresJsonObject),

            // Other value types just fine, usually it should be object though
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
