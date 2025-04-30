use ed25519_dalek::SigningKey;
use oz_keystore::LocalClient;
use stellar_strkey::ed25519::PublicKey;

fn main() {
    // generates a private key stored in encrypted json file
    LocalClient::generate(".".into(), "test".into(), Some("my-key.json"));

    // loads the private key using password
    let key_raw = LocalClient::load("./my-key.json".into(), "test".into());

    let key_bytes: [u8; 32] = key_raw
        .try_into()
        .expect("Invalid key length - must be 32 bytes");
    let keypair = SigningKey::from_bytes(&key_bytes);
    let pubkey = PublicKey(*keypair.verifying_key().as_bytes());

    println!("wallet: {:?}", pubkey.to_string())
}
