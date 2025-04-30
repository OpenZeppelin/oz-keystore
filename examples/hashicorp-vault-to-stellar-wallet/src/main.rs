use ed25519_dalek::SigningKey;
use oz_keystore::{HashicorpVaultClient, KeyType};
use rand::Rng;
use reqwest::ClientBuilder;
use stellar_strkey::ed25519::PublicKey;

#[tokio::main]
async fn main() {
    let client = ClientBuilder::new()
        .danger_accept_invalid_certs(true) // Allow self-signed certificates
        .build()
        .unwrap();

    let client = HashicorpVaultClient::new("http://127.0.0.1:8200", "root").with_client(client);

    let random_key: [u8; 32] = rand::thread_rng().gen();
    client
        .store_secret("my_stellar_secret", random_key.to_vec(), KeyType::Stellar)
        .await
        .unwrap();

    let secrets = client.list_secrets().await.unwrap();
    println!("all secrets {:?}", secrets);

    let secret = client
        .get_secret("my_stellar_secret", KeyType::Stellar)
        .await
        .unwrap();

    let key_bytes: [u8; 32] = secret
        .unwrap()
        .try_into()
        .expect("Invalid key length - must be 32 bytes");
    let keypair = SigningKey::from_bytes(&key_bytes);
    let pubkey = PublicKey(*keypair.verifying_key().as_bytes());

    println!("wallet: {:?}", pubkey.to_string())
}
