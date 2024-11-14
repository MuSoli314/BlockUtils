use digest::Digest;
use hex::decode;
use sha2::Sha256;
use base58::{ToBase58, FromBase58};
use sha3::Keccak256;

pub fn eth2trx(address: &str) -> String {
    let addr = address.replace("0x", "41");    
    let h = decode(addr).unwrap();    
    convert_b58encode(h)
}

pub fn convert_b58encode<T: AsRef<[u8]>>(raw: T) -> String {
    let mut hasher = Sha256::new();
    hasher.update(raw.as_ref());
    let digest1 = hasher.finalize();

    let mut hasher = Sha256::new();
    hasher.update(&digest1);
    let digest = hasher.finalize();

    let mut raw = raw.as_ref().to_owned();
    raw.extend(&digest[..4]);
    raw.to_base58()
}

pub fn convert_prefix(address: &str) -> String {
    address.replace("0x", "41")
}

pub fn trx2eth(addr: &str) -> Result<String, String> {
    let mut addr_vec_u8 = b58decode_check(addr)?;
    addr_vec_u8 = addr_vec_u8[1..].to_vec();
    let mut addr = format!("0x{}", hex::encode(addr_vec_u8));
    eip55_checksum(unsafe { &mut addr.as_bytes_mut()[2..] });
    
    Ok(addr.to_lowercase())
}

/// Base58check decode.
pub fn b58decode_check(s: &str) -> Result<Vec<u8>, String> {
    if let Ok(mut result) = s.from_base58() {
        let check = result.split_off(result.len() - 4);

        let mut hasher = Sha256::new();
        hasher.update(&result);
        let digest1 = hasher.finalize();

        let mut hasher = Sha256::new();
        hasher.update(&digest1);
        let digest = hasher.finalize();
        
        if check == &digest[..4] {
            return Ok(result);
        }
    }
    
    Err("The Address is invaild".to_string())
}

fn eip55_checksum(hex_address: &mut [u8]) {
    let mut hasher = Keccak256::new();
    hasher.update(&hex_address);
    let hashed_address = hex::encode(hasher.finalize());

    hex_address
        .iter_mut()
        .zip(hashed_address.as_bytes().iter())
        .for_each(|(c, &h)| match *c {
            b'a'..=b'f' if h > b'7' => {
                *c = c.to_ascii_uppercase();
            }
            _ => (),
        });
}

#[test]
fn test() {
    // TWjxTu8E5N4gDVySohe42E3pxPLhfCwzUE 地址
    let add = String::from("0x641725ed2b61cf433b0f60fa57372701e11c9f5e");
    println!("{:?}", eth2trx(&add));
    let trx_add = String::from("TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t");
    let eth_add = trx2eth(&trx_add);
    println!("eth_add: {:?}", eth_add)
}