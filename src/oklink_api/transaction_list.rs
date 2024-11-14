use serde_json::Value;

use super::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetTransactionListParams {
    #[serde(rename = "chainShortName")]
    pub chain_short_name: String,
    pub address: String,
    #[serde(rename = "protocolType")]
    pub protocol_type: String,
    #[serde(rename = "tokenContractAddress")]
    pub token_contract_address: String,
    #[serde(rename = "startBlockHeight")]
    pub start_block_height: u64,
    #[serde(rename = "endBlockHeight")]
    pub end_block_height: u64,
    #[serde(rename = "isFromOrTo")]
    pub is_from_or_to: String,
    pub page: u32,
    pub limit: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TransactionListData {
    pub page: String,
    pub limit : String,
    #[serde(rename = "totalPage")]
    pub total_page: String,
    #[serde(rename = "chainFullName")]
    pub chain_full_name: String,
    #[serde(rename = "chainShortName")]
    pub chain_short_name: String,
    // #[serde(rename = "transactionLists")]
    // pub transaction_lists: Vec<TransactionList>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionList {
    #[serde(rename = "txId")]
    pub tx_id: String,
    #[serde(rename = "methodId")]
    pub method_id: String,
    #[serde(rename = "blockHash")]
    pub block_hash: String,
    pub height: String,
    #[serde(rename = "transactionTime")]
    pub transaction_time: String,
    pub from: String,
    pub to: String,
    #[serde(rename = "isFromContract")]
    pub is_from_contract: bool,
    #[serde(rename = "isToContract")]
    pub is_to_contract: bool,
    pub amount: String,
    #[serde(rename = "transactionSymbol")]
    pub transaction_symbol: String,
    #[serde(rename = "txFee")]
    pub tx_fee: String,
    pub state: String,
    #[serde(rename = "tokenId")]
    pub token_id: String,
    #[serde(rename = "tokenContractAddress")]
    pub token_contract_address: String,
    #[serde(rename = "challengeStatus")]
    pub challenge_status: String,
    #[serde(rename = "l1OriginHash")]
    pub l1_origin_hash: String,
}

impl OkClient {
    // 最多返回近10000条数据，totalPage*limit最大10000
    pub async fn get_transaction_list(
        &self,
        params: GetTransactionListParams
    ) -> Result<OkApiData<Vec<TransactionListData>>, anyhow::Error> {
        let url = format!(
            "address/transaction-list?chainShortName={}&address={}&protocolType={}&tokenContractAddress={}&startBlockHeight={}&endBlockHeight={}&isFromOrTo={}&page={}&limit={}",
            params.chain_short_name, 
            params.address,
            params.protocol_type,
            params.token_contract_address,
            params.start_block_height,
            params.end_block_height,
            params.is_from_or_to,
            params.page,
            params.limit,
        );
        // println!("{}", url);

        let res = self.get(&url).await?;

        Ok(res)
    }
}
// 待完善
pub fn convert(post_json: Value) -> Result<String, anyhow::Error> {
    if let Some(object) = post_json.as_object() {
        // 遍历 JSON 对象的每个键值对
        let mut res = String::new();
        for (key, value) in object {
            println!("Key: {}", key);
            print!("Value: {}", value);
            res += &format!("{}={}", key, value)
        }
        return Ok(res)
    }

    Err(anyhow::anyhow!("post_json is not a object"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[async_std::test]
    async fn test_() {
        let ok_client = init_ok_client();

        let get_transaction_list_params = GetTransactionListParams {
            chain_short_name: "tron".to_string(),
            address: "TAzsQ9Gx8eqFNFSKbeXrbi45CuVPHzA8wr".to_string(),
            protocol_type: "token_20".to_string(), // 
            token_contract_address: "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t".to_string(),
            start_block_height: 0,
            end_block_height: 64650000,
            is_from_or_to: "to".to_string(), // to/from
            page: 1,
            limit: 1,
        };

        let val = serde_json::to_value(get_transaction_list_params).unwrap();
        println!("val: {:#?}", val);

        println!("{:?}", convert(val))

        // match ok_client.get_transaction_list(get_transaction_list_params).await {
        //     Ok(data) => {
        //         println!("{:#?}", data);
        //     }
        //     Err(err) => {
        //         println!("{:?}", err)
        //     }
        // }
    }
}
