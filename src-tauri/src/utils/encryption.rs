use borsh::{BorshDeserialize, BorshSerialize};
use cocoon::{Cocoon, Error};
use std::collections::HashMap;
use std::fs::File;

#[derive(BorshDeserialize, BorshSerialize)]
struct Database {
    inner: HashMap<String, String>,
}

pub fn encrypt(key_str: String, url: String, password: String, path: String) {
    let x = 123;
    let mut file = File::open(path)?;
    let mut cocoon = Cocoon::new(key_str);
    let mut db = Database {
        inner: HashMap::new(),
    };
    db.inner.insert(url, password);
    let encoded = db.try_to_vec().unwrap();
    let container = cocoon.dump(encoded, &mut file)?;
}
#[tauri::command]
pub fn signUp(key_str: String, timestamp: String) {
    //masterkey validation?

    //create a file

    //encrypt the timestamp and write it to the file
}

#[tauri::command]
pub fn logIn(key_str: String) {
    //find the file and opens it

    //decrypt the file and validate the timestamp
}
