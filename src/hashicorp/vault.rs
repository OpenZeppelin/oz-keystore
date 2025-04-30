use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use stellar_strkey::ed25519::PrivateKey;

#[derive(Serialize, Deserialize, Debug)]
struct SecretData {
    secret: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SecretRequest {
    data: SecretData,
}

#[derive(Deserialize, Debug)]
struct SecretResponseData {
    data: Option<SecretData>,
}

#[derive(Deserialize, Debug)]
struct SecretResponse {
    data: Option<SecretResponseData>,
}

#[derive(Deserialize, Debug)]
struct SecretListData {
    keys: Vec<String>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct SecretListResponse {
    request_id: String,
    lease_id: String,
    renewable: bool,
    lease_duration: u64,
    data: SecretListData,
    #[serde(default)]
    wrap_info: Option<()>,
    #[serde(default)]
    warnings: Option<()>,
    #[serde(default)]
    auth: Option<()>,
    mount_type: String,
}

#[derive(Debug)]
pub enum KeyType {
    EVM,
    Stellar,
    Solana,
    Any,
}

pub struct HashicorpVaultClient {
    client: Client,
    base_url: String,
    token: String,
}

impl HashicorpVaultClient {
    pub fn new(base_url: &str, token: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
            token: token.to_string(),
        }
    }

    pub fn with_client(&self, client: Client) -> Self {
        Self {
            client,
            base_url: self.base_url.clone(),
            token: self.token.clone(),
        }
    }

    fn encode_key(&self, key: Vec<u8>, key_type: KeyType) -> String {
        match key_type {
            KeyType::EVM => hex::encode(&key),
            KeyType::Stellar => PrivateKey::from_payload(&key).unwrap().to_string(),
            KeyType::Solana => bs58::encode(key).into_string(),
            KeyType::Any => String::from_utf8(key).unwrap(),
        }
    }

    fn decode_key(&self, key: String, key_type: KeyType) -> Vec<u8> {
        match key_type {
            KeyType::EVM => hex::decode(&key).unwrap(),
            KeyType::Stellar => PrivateKey::from_string(&key).unwrap().0.to_vec(),
            KeyType::Solana => bs58::decode(key).into_vec().unwrap(),
            KeyType::Any => key.into_bytes(),
        }
    }

    pub async fn store_secret(
        &self,
        id: &str,
        secret: Vec<u8>,
        key_type: KeyType,
    ) -> Result<(), Error> {
        let url = format!("{}/v1/secret/data/{}", self.base_url, id);

        let body = SecretRequest {
            data: SecretData {
                secret: self.encode_key(secret, key_type),
            },
        };

        self.client
            .post(&url)
            .header("X-Vault-Token", &self.token)
            .json(&body)
            .send()
            .await?;

        Ok(())
    }

    pub async fn list_secrets(&self) -> Result<Vec<String>, Error> {
        let url = format!("{}/v1/secret/metadata?list=true", self.base_url);
        let response = self
            .client
            .get(&url)
            .header("X-Vault-Token", &self.token)
            .send()
            .await?
            .json::<SecretListResponse>()
            .await?;

        Ok(response.data.keys)
    }

    pub async fn get_secret(&self, id: &str, key_type: KeyType) -> Result<Option<Vec<u8>>, Error> {
        let url = format!("{}/v1/secret/data/{}", self.base_url, id);
        let response: SecretResponse = self
            .client
            .get(&url)
            .header("X-Vault-Token", &self.token)
            .send()
            .await?
            .json()
            .await?;

        Ok(response
            .data
            .and_then(|d| d.data)
            .map(|d| self.decode_key(d.secret, key_type)))
    }

    pub async fn delete_secret(&self, id: &str) -> Result<(), Error> {
        let url = format!("{}/v1/secret/data/{}", self.base_url, id);
        self.client
            .delete(&url)
            .header("X-Vault-Token", &self.token)
            .send()
            .await?;

        Ok(())
    }
}
