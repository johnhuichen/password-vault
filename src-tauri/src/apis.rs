use std::process::Command;
use std::sync::Mutex;

use crate::vault::Vault;

#[tauri::command]
pub fn login(maybe_vault: tauri::State<Mutex<Option<Vault>>>, masterkey: &str) -> String {
    // vault can return an error because we can't login with the wrong masterkey
    let vault = Vault::new(masterkey.to_string()).expect("Should create vault");

    let mut test = maybe_vault.lock().expect("Should lock mutex");
    *test = Some(vault);
    println!("maybe_vault={maybe_vault:?}");
    // println!("vault={vault:?}");
    println!("test={test:?}");
    "123".to_string()
}
