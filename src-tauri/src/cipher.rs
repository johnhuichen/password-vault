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
