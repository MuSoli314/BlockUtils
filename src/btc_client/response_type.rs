use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonResponse<T> {
    pub error: Option<JsonError>,
    pub id: usize,
    pub result: Option<T>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonError {
    pub code: i32,
    pub message: String,
}

// getblock
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub hash: String,
    pub confirmations: i32,
    pub height: usize,
    pub version: i32,
    #[serde(rename="versionHex")]
    pub version_hex: Option<String>,
    pub merkleroot: String, 
    pub time: usize,
    pub mediantime: Option<usize>,
    pub nonce: u32,
    pub bits: String,
    pub difficulty: f64,
    pub chainwork: String,
    #[serde(rename="nTx")]
    pub n_tx: usize,
    pub previousblockhash: Option<String>,
    pub nextblockhash: Option<String>,
    pub strippedsize: Option<usize>,
    pub size: usize,
    pub weight: usize,
    pub tx: Vec<String>,
}

// getrawtransaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub in_active_chain: Option<bool>,
    pub txid: String,
    pub hash: String,
    pub version: u32,
    pub size: usize,
    pub vsize: usize,
    pub weight: usize, // new bitcoincore-rpc包多出来的字段
    pub locktime: u32,
    pub vin: Vec<Input>,
    pub vout: Vec<Output>,
    pub hex: String,
    pub blockhash: Option<String>,
    pub confirmations: Option<u32>,
    pub time: Option<usize>,
    pub blocktime: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Input {
    pub txid: Option<String>, // 出块奖励交易为空
    pub vout: Option<u32>, // 出块奖励交易为空
    #[serde(rename = "scriptSig")]
    pub script_sig: Option<ScriptSig>, // 出块奖励交易为空
    pub txinwitness: Option<Vec<String>>, // 出块奖励交易为空
    pub sequence: u32,
    pub coinbase: Option<String>, // 非出块奖励交易为空
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptSig {
    pub asm: String,
    pub hex: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Output {
    pub value: f64,
    pub n: u32,
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: ScriptPubKey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptPubKey {
    pub asm: String,
    pub desc: String, // new bitcoincore-rpc包多出来的字段
    pub hex: String,
    pub address: Option<String>,
    pub addresses: Option<Vec<String>>,
    pub r#type: Option<String>,  // `type` is a reserved keyword in Rust, hence the use of `r#`
    pub req_sigs: Option<usize>,
}