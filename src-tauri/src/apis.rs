use std::process::Command;
use std::sync::Mutex;

#[tauri::command]
pub fn login(process: tauri::State<Mutex<Command>>, master_key: &str) -> String {
    println!("master={master_key}");
    println!("process={process:?}");
    "123".to_string()
}
