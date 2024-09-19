// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use snafu::{ResultExt, Whatever};

use self::cipher::Cipher;
use self::vault::Vault;

mod cipher;
mod passwords;
mod vault;

#[snafu::report]
fn main() -> Result<(), Whatever> {
    // tauri::Builder::default()
    //     .run(tauri::generate_context!())
    //     .expect("error while running tauri application");

    let cipher = Cipher::new("masterkey");

    let vault = Vault::new(cipher).whatever_context("Cannot create a vault")?;
    vault
        .add_password("domain", "password")
        .whatever_context("Cannot add password")?;
    vault
        .update_password("domain", "another password")
        .whatever_context("cannot add second password")?;
    vault.view_passwords().whatever_context("whatever")?;

    println!("Try to delete something that does not exist");
    vault
        .delete_password("foobar")
        .whatever_context("whatever")?;
    vault.view_passwords().whatever_context("whatever")?;

    println!("Try to delete something that exists");
    vault
        .delete_password("domain")
        .whatever_context("whatever")?;
    vault.view_passwords().whatever_context("whatever")?;
    Ok(())
}
