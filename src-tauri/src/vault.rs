use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};

use borsh::{BorshDeserialize, BorshSerialize};
use snafu::{ResultExt, Snafu};

use crate::cipher::Cipher;
use crate::cipher::CipherError;
use crate::passwords::Passwords;

#[derive(Debug, Snafu)]
pub enum VaultError {
    #[snafu(display("IO Error: {:?}", source))]
    #[snafu(context(false))]
    IO { source: std::io::Error },

    #[snafu(display("Cipher Error: {:?}", source))]
    Cipher { source: CipherError },
}

#[derive(Debug, Snafu)]
pub enum UpdatePasswordError {}

#[derive(Debug, Snafu)]
pub enum DeletePasswordError {}

pub type Result<T> = std::result::Result<T, VaultError>;

#[derive(Debug)]
pub struct Vault {
    cipher: Cipher,
    path: PathBuf,
}

impl Vault {
    pub fn new(cipher: Cipher) -> Result<Self> {
        let path = Path::new("./passwd");
        let result = Vault {
            cipher,
            path: path.to_owned(),
        };

        if !path.exists() {
            File::create(path)?;
            result.save_to_file(Passwords::empty())?;
        }

        Ok(result)
    }

    pub fn view_passwords(&self) -> Result<()> {
        let passwords = self.decrypt_from_file()?;
        println!("{passwords:?}");
        Ok(())
    }

    pub fn add_password(&self, domain: &str, password: &str) -> Result<()> {
        let mut passwords = self.decrypt_from_file()?;

        if passwords.contains_key(domain) {
            println!("Domain '{}' already exists. No new password added.", domain);
            return Ok(());
        }

        passwords.insert(domain.to_string(), password.to_string());

        self.save_to_file(passwords)?;

        Ok(())
    }

    pub fn update_password(&self, domain: &str, password: &str) -> Result<()> {
        let mut passwords = self.decrypt_from_file()?;

        passwords.update(domain.to_string(), password.to_string());

        self.save_to_file(passwords)?;

        Ok(())
    }

    pub fn delete_password(&self, domain: &str) -> Result<()> {
        let mut passwords = self.decrypt_from_file()?;

        passwords.delete(domain);

        self.save_to_file(passwords)?;

        Ok(())
    }

    fn save_to_file(&self, passwords: Passwords) -> Result<()> {
        let file = File::create(&self.path)?;
        let mut writer = BufWriter::new(file);

        self.cipher
            .dump(passwords.try_to_vec()?, &mut writer)
            .context(CipherSnafu)?;

        Ok(())
    }

    fn decrypt_from_file(&self) -> Result<Passwords> {
        let file = File::open(&self.path)?;
        let mut reader = BufReader::new(file);
        let data: &[u8] = &self.cipher.parse(&mut reader).context(CipherSnafu)?;

        Ok(Passwords::try_from_slice(data)?)
    }
}
