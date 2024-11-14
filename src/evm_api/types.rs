use super::*;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EthApiData<T> {
    pub id: u32,
    pub jsonrpc: String,
    pub result: Option<T>,
    pub error: Option<Value>
}