// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use snafu::Whatever;

use self::apis::login;
use self::vault::Vault;

mod apis;
mod cipher;
mod passwords;
mod vault;

#[snafu::report]
fn main() -> Result<(), Whatever> {
    let maybe_vault: Option<Vault> = None;

    tauri::Builder::default()
        .manage(Mutex::new(maybe_vault))
        .invoke_handler(tauri::generate_handler![login])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
