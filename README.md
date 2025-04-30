## Keystore

A multi-chain keystore library that provides a unified interface for managing private keys.

> Warning: this is an experimental project under development.

## Features

- **Multiple Key Sources**
  - Local keystore management (JSON keystore files)
  - HashiCorp Vault integration (both on-prem vaults and cloud)
  
- **Chain Support**
  - Ethereum (EVM) compatible chains
  - Stellar/Soroban
  - Solana
  
- **Key Operations**
  - Generate new keys (local keystore only)
  - Load existing keys
  - Secure key storage and retrieval

## Examples

Check the `examples/` directory for complete usage examples:
- `local-keystore-to-alloy-wallet`: Convert keystore to EVM wallet
- `local-keystore-to-stellar-wallet`: Convert keystore to Stellar wallet
- `hashicorp-vault-to-alloy-wallet`: Convert pk stored in vault to EVM wallet
- `hashicorp-vault-to-stellar-wallet`: Convert pk stored in vault to Stellar wallet
- `hashicorp-vault-to-solana-wallet`: Convert pk stored in vault to Solana wallet
- `hashicorp-vault-any-secret`: Store and retrieve any secret in vault

## Local Keystore

The local keystore provides secure storage of private keys in encrypted JSON files on your local filesystem. It supports multiple key types (EVM, Stellar, Solana) and uses industry-standard encryption methods.

### Usage

See [local-keystore-to-alloy-wallet example](examples/hashicorp-vault-to-alloy-wallet) for full implementation using alloy.

```rust
  let dir = "./key";
  let password = "password123";
  let name = "key.json";
  let key = LocalClient::generate(dir, password, name)
```

## Hashicorp Vault

Provides integration with HashiCorp Vault for secure key management. Supports both local Vault instances and HashiCorp Cloud, with the ability to store and retrieve private keys for multiple blockchain networks (EVM, Stellar, Solana).

### Getting Started

1. Follow [Installation Guide](https://developer.hashicorp.com/vault/docs/install)
2. Open a terminal Run Vault locally
```bash
vault server -dev -dev-root-token-id="root"
```

### Usage

See [hashicorp-vault-to-alloy-wallet example](examples/hashicorp-vault-to-alloy-wallet) for full implementation.

```rust
  let random_key: [u8; 32] = rand::thread_rng().gen();
  client.store_secret("my_secret", random_key.to_vec(), KeyType::EVM).await.unwrap();

  let secret = client.get_secret("my_secret", KeyType::EVM).await.unwrap().unwrap();
  let hex_secret = hex::encode(&secret);
  let key_bytes = FixedBytes::from_hex(&hex_secret).unwrap();
  let signer = LocalSigner::from_bytes(&key_bytes)
    .expect("failed to create signer");
```

## Hashicorp Cloud

### Getting Started

1. Create a [Hashicorp account](https://portal.cloud.hashicorp.com/sign-in)
2. Create new organization
3. Go to [secrets app](portal.cloud.hashicorp.com/services/secrets/apps) and create a new app.
4. Create new static secret, the value must be a valid ed25519 secret key, you can generate a random key using https://cyphr.me/ed25519_tool/ed.html

### Setup

Create .env file with following entries

```bash
HASHICORP_CLIENT_ID=L5...Xa
HASHICORP_CLIENT_SECRET=Q9...2P
HASHICORP_ORG_ID=1b345678-b123-a123-c123-1b345678 # in org settings
HASHICORP_PROJECT_ID=1b345678-b123-a123-c123-1b345678 # in project settings
HASHICORP_APP_NAME=your_app_name
```

### Usage

> 

```rust
use keystore::hashicorp::cloud::HashicorpCloudClient;

// Initialize client with your credentials
let client = HashicorpCloudClient::new(
    env::var("HASHICORP_CLIENT_ID").unwrap(),
    env::var("HASHICORP_CLIENT_SECRET").unwrap(),
    env::var("HASHICORP_ORG_ID").unwrap(),
    env::var("HASHICORP_PROJECT_ID").unwrap(),
    env::var("HASHICORP_APP_NAME").unwrap(),
);

// Fetch a secret
let response = client.get_secret("my_secret_name").await?;
let secret_value = response.secret.static_version.value;
```