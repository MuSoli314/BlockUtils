use std::time::Duration;

use reqwest::{Client, header::{HeaderMap, HeaderValue}};
use serde::{Serialize, Deserialize, de::DeserializeOwned};

pub mod entity_label;
pub mod transaction_list;

#[derive(Debug, Clone)]
pub struct OkClient {
    pub client: Client,
    pub base_url: String,
    pub keys: Vec<String>,
    pub key_index: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OkApiData<T> {
    pub code: String,
    pub msg: String,
    pub data: Option<T>,
}

impl OkClient {
    pub fn new(base_url: &str, keys: Vec<String>, timeout: Duration) -> Result<Self, anyhow::Error> {
        if keys.is_empty() {
            return Err(anyhow::anyhow!("=====API keys is empty!!!======"))
        }
        
        let mut headers = HeaderMap::new();
        headers.insert("Ok-Access-Key", HeaderValue::from_str(&keys[0])?);

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(timeout)
            .build()?;

        Ok(OkClient { 
            client, 
            base_url: base_url.to_owned(),
            keys,
            key_index: 0,
        })
    }

    pub async fn get<T>(&self, join_url: &str) -> Result<OkApiData<Vec<T>>, anyhow::Error>
    where T: DeserializeOwned 
    {
        let url = format!("{}/{}", self.base_url, join_url);

        let res = self
            .client
            .get(url)
            .send()
            .await?
            .json::<OkApiData<Vec<T>>>()
            .await?;

        Ok(res)
    }

    pub fn update_api_key(&mut self) -> Result<(), anyhow::Error> {
        let key_index = self.key_index + 1;

        if key_index>=self.keys.len() {
            return Err(anyhow::anyhow!("=====All API keys are depleted!!!======"))
        }

        self.key_index = key_index;

        let mut headers = HeaderMap::new();
        headers.insert("Ok-Access-Key", HeaderValue::from_str(&self.keys[self.key_index])?);

        self.client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(())
    }
}

// 仅测试使用
pub fn init_ok_client() -> OkClient {
    let base_url = "https://www.oklink.com/api/v5/explorer";
    let keys = vec![
        "d98f33ab-54b7-4d05-a538-52dfe5764b52".to_owned()
    ];
    let timeout = Duration::from_secs(10);
    let ok_client = OkClient::new(base_url, keys, timeout).unwrap();

    ok_client
}


#[cfg(test)]
mod tests {
    use serde_json::Value;

    use super::*;

    #[async_std::test]
    async fn test_() {
        let ok_client = init_ok_client();

        match ok_client.get::<Value>("blockchain/summary?chainShortName=BTC").await {
            Ok(data) => println!("{:#?}", data),
            Err(err) => println!("{:#?}", err),
        }
    }
}