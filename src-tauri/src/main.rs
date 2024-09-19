// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;
use std::sync::Mutex;

use snafu::Whatever;

use self::apis::login;

mod apis;
mod cipher;
mod passwords;
mod vault;

#[snafu::report]
fn main() -> Result<(), Whatever> {
    let process = Command::new("powershell");
    tauri::Builder::default()
        .manage(Mutex::new(process))
        .invoke_handler(tauri::generate_handler![login])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
