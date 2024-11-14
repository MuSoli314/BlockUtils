use log::{error};
use serde::{Deserialize, Serialize};
use sha3::{Keccak256, Digest};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MethodInfo {
    pub constant: Option<bool>,
    pub name: Option<String>,
    pub inputs: Option<Vec<Parameter>>,
    pub outputs: Option<Vec<Parameter>>,
    pub r#type: String,
    #[serde(rename="stateMutability")]
    pub state_mutability: Option<String>,
    pub anonymous: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Parameter {
    pub components: Option<Vec<Parameter>>,
    pub indexed: Option<bool>,
    #[serde(rename="internalType")]
    pub internal_type: Option<String>,
    pub name: Option<String>,
    pub r#type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMetInfo {
    pub constant: Option<bool>,
    pub input: String,
    pub method_name: String,
    pub output: String,
    pub method_id: String,
    pub method_id_all: String,
    pub raw_data: MethodInfo,
}

pub fn find_abi_mets(abi: String) -> Result<Vec<GetMetInfo>, String> {
    if let Ok(abi_json) = serde_json::from_str::<Vec<MethodInfo>>(&abi) {
        // println!("==abi_json len: {}", abi_json.len());
        let mets = abi_json.into_iter().filter_map(|met_info|{
            get_method_id(&met_info)
        }).collect::<Vec<_>>();

        return Ok(mets);
    } else {
        let err = format!("Vec<GetMetInfo> Match Abi Error!!! abi:{} abi: {:?}", abi, abi);
        return Err(err);
    }
}

pub fn get_method_id(method_info: &MethodInfo) -> Option<GetMetInfo> {
    // 确定参数
    if let Some(name) = &method_info.name { // name存在
        let mut met_info = GetMetInfo {
            method_id: "Null".to_string(),
            method_name: name.clone(),
            input: "()".to_string(),
            output: "Null".to_string(),
            constant: constant_flag(method_info),
            method_id_all: "Null".to_string(),
            raw_data: method_info.clone(),
        };
        // input 存在
        if let Some(inputs) = &method_info.inputs{ 
            let method_in_pars = get_parameters(inputs.clone());
            met_info.input = method_in_pars;
        }
        // output 存在
        if let Some(outputs) = &method_info.outputs{ 
            let method_out_pars = get_parameters(outputs.clone());
            met_info.output = method_out_pars;
        }
        // 计算方法id
        let method_input = format!("{}{}", name, met_info.input);
        let hex = str2hex(method_input.clone());
        let method_id = format!("0x{}", &hex[..8]);
        met_info.method_id = method_id;
        met_info.method_id_all = format!("0x{}", hex);
        return Some(met_info);
    }
    return None;
}

// 获取函数的输入/输出参数
pub fn get_parameters(inputs: Vec<Parameter>) -> String {
    let parameters = inputs.into_iter().map(|par|{
        let par_str = if let Some(components) = par.components{
            let parameter = get_parameters(components);
            parameter
        }else{
            par.r#type
        };
        par_str
    }).collect::<Vec<_>>();
    format!("({})", parameters.join(","))
}

// 判断该函数是否是常量(简单验证，未通过验证的为None)
pub fn constant_flag(method_info: &MethodInfo) -> Option<bool> {
    let mut constant = Option::None;
    if let Some(t) = method_info.constant{
        constant = Some(t);
    } else if method_info.r#type=="event" {
        constant = Some(false);
    } else if method_info.state_mutability.is_some() {
        let state_mutability = method_info.state_mutability.clone().unwrap();
        match state_mutability.to_lowercase().as_str() {
            "pure" => constant = Some(true),
            "view" => constant = Some(true),
            "payable" => constant = Some(false),
            "nonpayable" => constant = Some(false),
            _ => { // 未知情况
                error!("===000===state_mutability: {} {:#?}", state_mutability, method_info)
            }
        }
    }
    constant
}

pub fn str2hex(str: String) -> String {
    let mut hasher = Keccak256::new();
    hasher.update(str.as_bytes());
    let result = hasher.finalize();
    let hex = hex::encode(result);
    // println!("len:{} str: {} result: {:?}", hex::encode(result).len(), str, hex::encode(result));
    hex
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    pub fn test() {
        // let mut abi_usdt_file = std::fs::File::open("src/abi_test/abi_usdt.txt").unwrap();
        // let mut abi_components_file = std::fs::File::open("src/abi_test/abi_components.txt").unwrap();
        // // 创建一个可变字符串来存储文件内容
        // let mut _abi_usdt = String::new();
        // let mut _abi_components = String::new();
        
        // let _t = std::io::Read::read_to_string(&mut abi_usdt_file, &mut _abi_usdt).unwrap();
        // let _t = std::io::Read::read_to_string(&mut abi_components_file, &mut _abi_components).unwrap();

        // let mets = find_abi_mets(_abi_usdt);
        // // println!("mets:{:#?}", mets);

        // // 创建文件并打开写入模式
        // let mut file = std::fs::File::create("src/abi_test/abi_usdt.json").unwrap();
        // let res = serde_json::to_string(&mets).unwrap();
        // std::io::Write::write_all(&mut file, res.as_bytes()).unwrap();

        let res = str2hex("AddedBlackList(address)".to_string());
        println!("{}", res);


        // let abi_test = r#"[{
        //     "anonymous": false,
        //     "inputs": [],
        //     "name": "Pause",
        //     "type": "event"
        // }]"#.to_string();
        // let mets = find_abi_mets(abi_test);
        // println!("mets:{:#?}", mets);
    }
}