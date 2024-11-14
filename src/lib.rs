pub mod db;
pub mod evm_api;
pub mod oklink_api;
pub mod btc_client;
pub mod utils;
pub mod worker;
pub mod init_app;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
