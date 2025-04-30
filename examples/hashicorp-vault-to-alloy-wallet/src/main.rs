use alloy_network::EthereumWallet;
use alloy_primitives::{hex::FromHex, FixedBytes};
use alloy_signer_local::LocalSigner;
use oz_keystore::{HashicorpVaultClient, KeyType};
use rand::Rng;
use reqwest::ClientBuilder;

#[tokio::main]
async fn main() {
    let client = ClientBuilder::new()
        .danger_accept_invalid_certs(true) // Allow self-signed certificates
        .build()
        .unwrap();

    let client = HashicorpVaultClient::new("http://127.0.0.1:8200", "root").with_client(client);

    let random_key: [u8; 32] = rand::thread_rng().gen();
    client
        .store_secret("my_secret", random_key.to_vec(), KeyType::EVM)
        .await
        .unwrap();
    let secret = client
        .get_secret("my_secret", KeyType::EVM)
        .await
        .unwrap()
        .unwrap();

    let hex_secret = hex::encode(&secret);

    let key_bytes = FixedBytes::from_hex(&hex_secret).unwrap();
    let signer = LocalSigner::from_bytes(&key_bytes).expect("failed to create signer");
    let wallet = EthereumWallet::from(signer);
    println!("wallet: {:?}", wallet);
}
