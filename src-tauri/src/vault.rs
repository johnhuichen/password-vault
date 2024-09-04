use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::vec;

use cocoon::Cocoon;
use snafu::{Location, ResultExt, Snafu};

use crate::cipher::Cipher;

#[derive(Debug, Snafu)]
pub enum AddPasswordError {
    #[snafu(display("IO Error"))]
    IO { source: std::io::Error },
}

#[derive(Debug, Snafu)]
pub enum UpdatePasswordError {}

#[derive(Debug, Snafu)]
pub enum DeletePasswordError {}

#[derive(Debug)]
pub struct Vault {
    cipher: Cipher,
}

impl Vault {
    pub fn new(cipher: Cipher) -> Result<Self, std::io::Error> {
        Ok(Vault { cipher })
    }

    pub fn add_password(&self, domain: &str, password: &str) -> Result<(), AddPasswordError> {
        let mut passwords = self.decrypt_from_file().context(IOSnafu)?;

        // need to make sure domain doesn't already exist

        passwords.insert(domain.to_string(), password.to_string());

        self.save_to_file(passwords).context(IOSnafu)?;

        Ok(())
    }

    pub fn update_password(&self, domain: &str, password: &str) -> Result<(), UpdatePasswordError> {
        todo!()
    }

    pub fn delete_password(&self, domain: &str, password: &str) -> Result<(), DeletePasswordError> {
        todo!()
    }

    fn save_to_file(&self, passwords: HashMap<String, String>) -> Result<(), std::io::Error> {
        todo!()
    }

    fn decrypt_from_file(&self) -> Result<HashMap<String, String>, std::io::Error> {
        let path = Path::new("./passwd");
        if !path.exists() {
            File::create(path)?;
        }
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        // TODO: need to use cipher to decrypt file
        // let test: HashMap<String, String>= reader
        //     .lines()
        //     .map(|line| {
        //         let line = line.unwrap_or_else(|e| panic!("Cannot get line: {e}"));
        //         let (domain, password) = line
        //             .split_once(":")
        //             .unwrap_or_else(|| panic!("Cannot parse {line}"));
        //         VaultEntry {
        //             domain: domain.to_string(),
        //             password: password.to_string(),
        //         }
        //     })
        //     .collect();

        Ok(HashMap::new())
    }
}
