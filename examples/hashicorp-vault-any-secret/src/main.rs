use oz_keystore::{HashicorpVaultClient, KeyType};
use reqwest::ClientBuilder;

#[tokio::main]
async fn main() {
    let client = ClientBuilder::new()
        .danger_accept_invalid_certs(true) // Allow self-signed certificates
        .build()
        .unwrap();

    let client = HashicorpVaultClient::new("http://127.0.0.1:8200", "root").with_client(client);

    let my_secret = "my_secret_string".to_string();
    client
        .store_secret("my_secret", my_secret.into_bytes(), KeyType::Any)
        .await
        .unwrap();
    let secret = client
        .get_secret("my_secret", KeyType::Any)
        .await
        .unwrap()
        .unwrap();

    println!("secret: {}", String::from_utf8(secret).unwrap());
}
