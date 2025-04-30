use reqwest::{Client, Error};
use serde::Deserialize;

pub struct HashicorpCloudClient {
    client: Client,
    client_id: String,
    client_secret: String,
    org_id: String,
    project_id: String,
    app_name: String,
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
}

#[derive(Debug, Deserialize)]
pub struct StaticVersion {
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct HashicorpSecret {
    pub static_version: StaticVersion,
}

#[derive(Debug, Deserialize)]
pub struct HashicorpResponse {
    pub secret: HashicorpSecret,
}

impl HashicorpCloudClient {
    pub fn new(
        client_id: String,
        client_secret: String,
        org_id: String,
        project_id: String,
        app_name: String,
    ) -> Self {
        Self {
            client: Client::new(),
            client_id,
            client_secret,
            org_id,
            project_id,
            app_name,
        }
    }

    pub fn with_client(&self, client: Client) -> Self {
        Self {
            client,
            client_id: self.client_id.clone(),
            client_secret: self.client_secret.clone(),
            org_id: self.org_id.clone(),
            project_id: self.project_id.clone(),
            app_name: self.app_name.clone(),
        }
    }

    async fn get_token(&self) -> Result<String, Error> {
        let token_response = self
            .client
            .post("https://auth.idp.hashicorp.com/oauth2/token")
            .form(&[
                ("client_id", &self.client_id),
                ("client_secret", &self.client_secret),
                ("grant_type", &String::from("client_credentials")),
                ("audience", &String::from("https://api.hashicorp.cloud")),
            ])
            .send()
            .await?
            .json::<TokenResponse>()
            .await?;

        Ok(token_response.access_token)
    }

    pub async fn get_secret(&self, secret_name: &str) -> Result<HashicorpResponse, Error> {
        let token = self.get_token().await?;

        let url = format!(
            "https://api.cloud.hashicorp.com/secrets/2023-11-28/organizations/{}/projects/{}/apps/{}/secrets/{}:open",
            self.org_id, self.project_id, self.app_name, secret_name
        );

        self.client
            .get(url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?
            .json()
            .await
    }
}
