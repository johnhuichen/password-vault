// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use snafu::{ResultExt, Whatever};

use self::cipher::Cipher;
use self::vault::Vault;

mod cipher;
mod vault;

#[snafu::report]
fn main() -> Result<(), Whatever> {
    // tauri::Builder::default()
    //     .run(tauri::generate_context!())
    //     .expect("error while running tauri application");

    let cipher = Cipher::new("masterkey");
    // let encrypted = cipher.encrypt("test123:pass123").unwrap();
    // println!("encrypted: {encrypted:?}");
    // let decrypted = cipher.decrypt(&encrypted);
    // println!("decrypted: {decrypted:?}");
    let vault = Vault::new(cipher).whatever_context("Cannot create a vault")?;
    vault
        .add_password("domain", "password")
        .whatever_context("Cannot add password")?;

    // if let Err(e) = vault.encrypt("domain", "password") {
    //     println!("{:?}", e);
    // }

    Ok(())
}
