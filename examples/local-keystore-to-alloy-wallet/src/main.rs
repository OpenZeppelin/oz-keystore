use alloy_network::EthereumWallet;
use alloy_primitives::FixedBytes;
use alloy_signer_local::LocalSigner;
use oz_keystore::LocalClient;

fn main() {
    // generates a private key stored in encrypted json file
    LocalClient::generate(".".into(), "test".into(), Some("my-key.json"));

    // loads the private key using password
    let key_raw = LocalClient::load("./my-key.json".into(), "test".into());

    // transforms the key into alloy wallet
    let key_bytes = FixedBytes::from_slice(&key_raw);
    let signer = LocalSigner::from_bytes(&key_bytes).expect("failed to create signer");
    let wallet = EthereumWallet::from(signer);
    println!("wallet: {:?}", wallet);
}
