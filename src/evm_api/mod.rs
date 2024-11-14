use std::time::Duration;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use self::types::EthApiData;

pub mod types;

#[derive(Debug, Clone)]
pub struct EvmNode {
    client: Client,
    pub rpcs: Vec<String>,
    pub rpcs_index: usize,
}

impl EvmNode {
    pub fn new(rpcs: Vec<String>, timeout: u64) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(timeout))
            .build()
            .unwrap();

        EvmNode {
            client,
            rpcs,
            rpcs_index: 0,
        }
    }

    pub fn update_index(&mut self) {
        self.rpcs_index = (self.rpcs_index + 1) % self.rpcs.len();
    }

    pub async fn http<T: for<'de> Deserialize<'de>>(
        &mut self,
        post_json: Value,
        try_count: Option<usize>
    ) -> Result<EthApiData<T>, anyhow::Error> {
        let mut count = 1;

        loop {
            let res = self
                .client
                .post(self.rpcs[self.rpcs_index].clone())
                .json(&post_json)
                .send()
                .await?
                .json::<EthApiData<T>>()
                .await;
            
            match res {
                Ok(data) => return Ok(data),
                Err(err) => {
                    println!("--EvmNode--index: {} try_count {} failed: {}. sleep 1s", self.rpcs_index, count, err);
                    // 达到最大尝试次数, 返回最后一次错误
                    if try_count.is_some_and(|try_count|try_count <= count) {
                        return Err(err.into());
                    }
                    async_std::task::sleep(Duration::from_secs(1)).await;
                },
            }

            self.update_index();

            count += 1;
        }
    }

    pub async fn eth_block_number(&mut self, try_count: Option<usize>) -> Result<EthApiData<String>, anyhow::Error> {
        let post_json = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "method": "eth_blockNumber",
            "params": [],
        });

        self.http(post_json, try_count).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[async_std::test]
    async fn test_() {
        let urls = vec![
            "http://192.168.195.239:50545/jsonc".to_string(),
            "http://192.168.195.239:50545/jsonrpc".to_string(),
        ];
        let mut evm_node = EvmNode::new(urls, 10);

        match evm_node.eth_block_number(Some(3)).await {
            Ok(data) => println!("{:#?}", data),
            Err(err) => println!("{}", err),
        }
    }
}
