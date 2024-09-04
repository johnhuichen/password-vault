use cocoon::Cocoon;

#[derive(Debug)]
pub struct Cipher {
    masterkey: String,
}

impl Cipher {
    pub fn new(masterkey: &str) -> Self {
        Cipher {
            masterkey: masterkey.to_string(),
        }
    }

    pub fn decrypt(&self, data: &[u8]) -> Result<String, cocoon::Error> {
        let cocoon = Cocoon::new(self.masterkey.as_bytes());
        cocoon.unwrap(data).map(|b| {
            String::from_utf8(b).unwrap_or_else(|e| panic!("Cannot parse string from data: {e}"))
        })
    }

    pub fn encrypt(&self, data: &str) -> Result<Vec<u8>, cocoon::Error> {
        let data = data.as_bytes();
        let mut cocoon = Cocoon::new(self.masterkey.as_bytes());
        cocoon.wrap(data)
    }
}

// pub fn encrypt(key_str: &str, url: &str, password: &str, path: &str) -> Result<(), EncryptError> {
//     // what happens if path doesn't exist
//     let mut file = File::open(path)?;
//
//     // let mut cocoon = Cocoon::new(key_str);
//     // let mut db = Database {
//     //     inner: HashMap::new(),
//     // };
//     // db.inner.insert(url, password);
//     // let encoded = db.try_to_vec().unwrap();
//     // let container = cocoon.dump(encoded, &mut file)?;
//     //
//     Ok(())
// }
// // #[tauri::command]
// // pub fn signUp(key_str: String, timestamp: String) {
// //     //masterkey validation?
// //
// //     //create a file
// //
// //     //encrypt the timestamp and write it to the file
// // }
// //
// // #[tauri::command]
// // pub fn logIn(key_str: String) {
// //     //find the file and opens it
// //
// //     //decrypt the file and validate the timestamp
// // }
