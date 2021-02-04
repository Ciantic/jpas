use derive_more::From;
use std::{
    io::Write,
    process::{Command, Stdio},
};

#[derive(From, Debug)]
pub enum Error {
    Io(std::io::Error),
    ExitCode(i32, String),
    SignalTerminated,
}

pub fn decrypt(str: &str) -> Result<String, Error> {
    let mut cmd = Command::new("gpg")
        .arg("--no-tty")
        .arg("--batch")
        .arg("--decrypt")
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()?;
    let outstdin = cmd.stdin.as_mut().unwrap();
    write!(outstdin, "{}", str)?;
    let result = cmd.wait_with_output().unwrap();
    match result.status.code() {
        Some(0) => Ok(String::from_utf8_lossy(&result.stdout).into()),
        Some(err) => Err(Error::ExitCode(
            err,
            String::from_utf8_lossy(&result.stderr).into(),
        )),
        None => Err(Error::SignalTerminated),
    }
}

pub fn encrypt(str: &str) -> Result<String, Error> {
    let mut cmd = Command::new("gpg")
        .arg("--no-tty")
        .arg("--batch")
        .arg("--default-recipient-self")
        .arg("--armor")
        .arg("--sign")
        .arg("--encrypt")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()?;
    let outstdin = cmd.stdin.as_mut().unwrap();
    write!(outstdin, "{}", str)?;
    let result = cmd.wait_with_output().unwrap();
    match result.status.code() {
        Some(0) => Ok(String::from_utf8_lossy(&result.stdout).into()),
        Some(err) => Err(Error::ExitCode(
            err,
            String::from_utf8_lossy(&result.stderr).into(),
        )),
        None => Err(Error::SignalTerminated),
    }
}

#[cfg(test)]
pub mod tests {
    use std::path::PathBuf;

    use normpath::PathExt;

    use super::{decrypt, encrypt};

    pub fn init_gpg_test() {
        // NOTE: We need normpath crate, because the canonicalization in Windows
        // adds UNC prefix, which fails to work with GPG4Win

        // Initiate testing GNUPGHOME if not existing yet
        let testkey = PathBuf::from("./tests/gpg/test-private.keys")
            .normalize()
            .unwrap();
        let gnupghome = PathBuf::from("./tests/gpg/.gnupghome");
        if std::fs::metadata(&gnupghome).is_err() {
            std::fs::create_dir(&gnupghome).unwrap();
            std::env::set_var("GNUPGHOME", gnupghome.normalize().unwrap());
            std::process::Command::new("gpg")
                .arg("--batch")
                .arg("--passphrase")
                .arg("")
                .arg("--import")
                .arg(testkey)
                .status()
                .expect("Failed to import the keys");
        } else {
            std::env::set_var("GNUPGHOME", gnupghome.canonicalize().unwrap());
        }
    }

    #[test]
    pub fn test_encrypt() {
        init_gpg_test();
        let value = encrypt("Foo").unwrap();
        assert_eq!(value.starts_with("-----BEGIN PGP MESSAGE-----"), true)
    }

    #[test]
    pub fn test_decrypt() {
        init_gpg_test();
        let value = encrypt("swordfish").unwrap();
        let again = decrypt(&value).unwrap();
        assert_eq!(again, "swordfish")
    }
}
