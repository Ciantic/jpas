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
        .stderr(Stdio::piped())
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
    use super::super::tests::test_init_gpghome;
    use super::{decrypt, encrypt};

    #[test]
    fn test_encrypt() {
        test_init_gpghome();
        let value = encrypt("swordfish").unwrap();
        assert_eq!(value.starts_with("-----BEGIN PGP MESSAGE-----"), true)
    }

    #[test]
    fn test_decrypt() {
        test_init_gpghome();
        let value = encrypt("swordfish").unwrap();
        let again = decrypt(&value).unwrap();
        assert_eq!(again, "swordfish")
    }
}
