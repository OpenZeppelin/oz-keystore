use std::path::PathBuf;

pub struct LocalClient {}

impl LocalClient {
    pub fn generate(dir: PathBuf, password: String, name: Option<&str>) -> (Vec<u8>, String) {
        let mut rng = rand::thread_rng();
        let result = eth_keystore::new(dir, &mut rng, password, name);
        result.unwrap()
    }

    pub fn update(dir: PathBuf, password: String, name: Option<&str>, pk: &[u8]) -> String {
        let result = eth_keystore::encrypt_key(dir, &mut rand::thread_rng(), pk, password, name);
        result.unwrap()
    }

    pub fn load(dir: PathBuf, password: String) -> Vec<u8> {
        eth_keystore::decrypt_key(dir, password).unwrap()
    }
}
