use anyhow::Result;
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Client,
};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{json, Value};
use std::time::Duration;

use self::response_type::*;

pub mod response_type;

#[derive(Debug, Clone)]
pub struct BtcClient {
    client: Client,
    url: String,
    headers: HeaderMap,
}

impl BtcClient {
    // 新建
    pub fn new(
        url: &str,
        username: &str,
        password: &str,
        timeout: Duration,
    ) -> Result<Self, anyhow::Error> {
        // 构造基本认证头
        let mut headers: HeaderMap = HeaderMap::new();
        // headers.insert(CONTENT_TYPE, HeaderValue::from_str("application/json")?);
        let auth = format!(
            "Basic {}",
            base64::encode(format!("{}:{}", username, password))
        );
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&auth)?);

        let client = reqwest::Client::builder().timeout(timeout).build()?;

        Ok(BtcClient {
            client,
            url: url.to_owned(),
            headers,
        })
    }
    // 通用post请求
    pub async fn http_post<T, U>(&self, post_json: T) -> Result<U, anyhow::Error>
    where
        T: Serialize,
        U: DeserializeOwned,
    {
        let res = self
            .client
            .post(&self.url)
            .headers(self.headers.clone())
            .json(&post_json)
            .send()
            .await?
            .json::<U>()
            .await?;

        Ok(res)
    }

    pub async fn get_block_count(&self) -> Result<JsonResponse<i32>, anyhow::Error> {
        let post_json = json!({
            "jsonrpc": "2.0",
            "id": 1, // Use the index as id, so we can quickly sort the response
            "method": "getblockcount",
            "params": []
        });

        log::debug!("[getblockhash] {:?}", post_json);

        let res = self.http_post::<Value, JsonResponse<i32>>(post_json).await?;

        Ok(res)
    }

    pub async fn get_block_hash(&self, height: u64) -> Result<JsonResponse<String>, anyhow::Error> {
        let post_json = json!({
            "jsonrpc": "2.0",
            "id": 1, // Use the index as id, so we can quickly sort the response
            "method": "getblockhash",
            "params": [ height ]
        });

        log::debug!("[getblockhash] {:?}", post_json);

        let res = self.http_post::<Value, JsonResponse<String>>(post_json).await?;

        Ok(res)
    }

    pub async fn get_block(&self, hash: &str) -> Result<JsonResponse<Block>, anyhow::Error> {
        let post_json = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getblock",
            "params": [hash]
        });

        log::debug!("[getblock] {:?}", post_json);

        let res = self.http_post::<Value, JsonResponse<Block>>(post_json).await?;

        Ok(res)
    }

    pub async fn get_raw_transaction(&self, hash: &str) -> Result<JsonResponse<Transaction>, anyhow::Error> {
        let post_json = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getrawtransaction",
            "params": [hash, true]
        });

        log::debug!("[getrawtransaction] {:?}", post_json);

        let res = self.http_post::<Value, JsonResponse<Transaction>>(post_json).await?;

        Ok(res)
    }

    pub async fn get_txs_raw(&self, hashs: Vec<String>) -> Result<Vec<JsonResponse<Transaction>>, anyhow::Error> {
        let post_json = hashs.into_iter().enumerate().map(|(index, hash)|{
            json!({
                "jsonrpc": "2.0",
                "id": index,
                "method": "getrawtransaction",
                "params": [hash, true]
            })
        }).collect::<Vec<_>>();

        log::debug!("[get_txs_raw] {:?}", post_json);

        let res = self.http_post::<Vec<Value>, Vec<JsonResponse<Transaction>>>(post_json).await?;

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn test_get_latest_block() -> Result<(), anyhow::Error> {
        let url = "http://192.168.195.233:8030";

        let btc_client = BtcClient::new(url, "admin", "admin", Duration::from_secs(10))?;

        let _txs = vec![
            "1744260c3affa4b4c332a7605056981fdd77b4e1858fc60eff8810418ba205f1".to_string(),
            "e2323e547149024551ac4ebec91f9fe4deafe8ea710b61383e1ad6cbb9448605".to_string()
        ];

        let res = btc_client
            .get_block_count()
            // .get_block_hash(8080080)
            // .get_block("0000000000000000000211eb82135b8f5d8be921debf8eff1d6b38b73bc03834")
            // .get_raw_transaction("01f8255cad4f0060170f5cca22a5e0ca99fa62b3d26b89a34ac3100d876d14a4")
            // .get_txs_raw(_txs)
            .await;

        match res {
            Ok(data) => {
                println!("{:#?}", data);
            },
            Err(err) => println!("{:#?}", err),
        }

        Ok(())
    }

    #[async_std::test]
    async fn ttt() {
        if let Err(err) = test_get_latest_block().await {
            println!("====={:?}", err);
        }
    }
}
// getblockhash
// curl --user user:pwd --data-binary '{"jsonrpc": "1.0", "id": "curltest", "method": "getblockhash", "params": [827692]}' -H 'content-type: text/plain;' http://192.168.195.233:8030 -o JSON/getblockhash.json

// getblock
// curl --user admin:admin --data-binary '{"jsonrpc": "1.0", "id": "curltest", "method": "getblock", "params": ["0000000000000000000211eb82135b8f5d8be921debf8eff1d6b38b73bc03834"]}' -H 'content-type: text/plain;' http://192.168.195.233:8030 -o JSON/getblock.json

// curl --user admin:admin --data-binary '{"jsonrpc": "1.0", "id": "curltest", "method": "getrawtransaction", "params": ["01f8255cad4f0060170f5cca22a5e0ca99fa62b3d26b89a34ac3100d876d14a4", true]}' -H 'content-type: text/plain;' 192.168.195.233:8030 -o jSON/getrawtransaction_miner.json
