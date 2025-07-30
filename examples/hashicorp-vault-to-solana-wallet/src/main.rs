use oz_keystore::{HashicorpVaultClient, KeyType};
use reqwest::ClientBuilder;
use solana_keypair::{keypair_from_seed, Keypair};
use solana_signer::Signer;

#[tokio::main]
async fn main() {
    let client = ClientBuilder::new()
        .danger_accept_invalid_certs(true) // Allow self-signed certificates
        .build()
        .unwrap();

    let client = HashicorpVaultClient::new("http://127.0.0.1:8200", "root").with_client(client);

    let keypair = Keypair::new();
    client
        .store_secret(
            "my_solana_secret",
            keypair.secret_bytes().to_vec(),
            KeyType::Solana,
        )
        .await
        .unwrap();

    let secret = client
        .get_secret("my_solana_secret", KeyType::Solana)
        .await
        .unwrap()
        .unwrap();
    let keypair = keypair_from_seed(&secret).unwrap();

    println!("solana key {:?}", keypair.pubkey());
}
