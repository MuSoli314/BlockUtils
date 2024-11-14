use std::{str::FromStr, time::Duration};

use reqwest::{Client, header::{HeaderMap, HeaderValue, HeaderName}};
use serde::{Serialize, de::DeserializeOwned};

#[derive(Debug, Clone)]
pub struct SelfClient {
    pub client: Client,
    pub base_url: String,
}

impl SelfClient {
    /// Create a new SelfClient. Params: base_url, header_vec, timeout
    pub fn new(base_url: &str, header_vec: Vec<(String, String)>, timeout: Option<Duration>) -> Result<Self, anyhow::Error> {
        let mut headers = HeaderMap::new();

        for (key, val) in header_vec {
            let header_name = HeaderName::from_str(&key)?;
            let header_value = HeaderValue::from_str(&val)?;
            headers.insert(header_name, header_value);
        }
        
        let mut client_builder = reqwest::Client::builder().default_headers(headers);
        if let Some(timeout) = timeout {
            client_builder = client_builder.timeout(timeout);
        }
        let client = client_builder.build()?;
    
        Ok(SelfClient { 
            client, 
            base_url: base_url.to_owned()
        })
    }
    
    pub async fn http_get<T>(&self, join_url: Option<String>) -> Result<T, anyhow::Error> 
    where 
        T: DeserializeOwned 
    {
        let mut url = self.base_url.clone();
        if let Some(join_url) = join_url {
            url += &join_url
        }

        let res = self
            .client
            .get(url)
            .send()
            .await?
            .json::<T>()
            .await?;

        Ok(res)
    }
    
    pub async fn http_post<T, U>(self, join_url: Option<String>, post_json: T) -> Result<U, anyhow::Error> 
    where 
        T: Serialize,
        U: DeserializeOwned
    {
        let mut url = self.base_url.clone();
        if let Some(join_url) = join_url {
            url += &join_url
        }

        let res = reqwest::Client::new()
            .post(&self.base_url)
            .json(&post_json)
            .send()
            .await?
            .json::<U>()
            .await?;
    
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use serde_json::{Value, json};

    use super::*;
    
    #[derive(Debug, Clone, Deserialize)]
    struct EvmApiData<T> {
        pub id: i32,
        pub jsonrpc: String,
        pub result: T,
    }

    #[derive(Debug, Clone, Deserialize)]
    struct EvmApiDataResult {
        pub number: String,
        pub hash: String,
        pub transactions: Vec<String>,
    }

    #[async_std::test]
    async fn test() {
        let timeout = Some(Duration::from_secs(1));
        // ok_client
        // let header_vec = vec![
        //     (
        //         "OK-ACCESS-KEY".to_owned(),
        //         "40c23c4a-74ca-4dab-9f65-37c2c384f5dc".to_owned()
        //     )
        // ];
        // let base_url = "https://www.oklink.com/api/v5/explorer/";
        // let ok_client = SelfClient::new(base_url, header_vec, timeout).unwrap();
        // let add_url = "blockchain/summary?chainShortName=BTC".to_string();
        // match ok_client.http_get::<Value>(Some(add_url)).await {
        //     Ok(data) => println!("{:#?}", data),
        //     Err(err) => println!("{:#?}", err),
        // }

        // evm api
        let base_url = "http://192.168.195.239:50545/jsonrpc";
        let ok_client = SelfClient::new(base_url, vec![], timeout).unwrap();

        let val = json!({
            "jsonrpc": "2.0",
            "method": "eth_getBlockByNumber",
            "params": [
                "0xaaa1b4",
                false
            ],
            "id": 1
        });

        match ok_client.http_post::<Value, EvmApiData<EvmApiDataResult>>(None, val).await {
            Ok(data) => println!("{:#?}", data),
            Err(err) => println!("{:#?}", err),
        }
    }
}

// 添加头部村砸问题