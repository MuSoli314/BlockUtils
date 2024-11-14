use config::{Config, File};
use serde::Deserialize;

// #[derive(Debug, Clone, Deserialize)]
// struct ConfigTest {
//     pub title: String,
//     pub log_info: LogInfo,
//     pub db_info: DbInfo,
// }

// #[derive(Debug, Clone, Deserialize)]
// struct LogInfo {
//     pub level: String,
//     pub dir_name: String,
//     pub file_name: String,
// }

// #[derive(Debug, Clone, Deserialize)]
// struct DbInfo {
//     pub host: String,
//     pub port: u16,
//     pub user: String,
//     pub password: String,
//     pub usdt_db: String,
//     pub min_connections: u32,
//     pub max_connections: u32,
// }

pub fn get_config<T: for<'a> Deserialize<'a>>(path: &str) -> Result<T, anyhow::Error> {
    let config = Config::builder()
        .add_source(File::with_name(path))
        .build()?
        .try_deserialize::<T>()?;

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_() {
        match get_config::<serde_json::Value>("config.toml") {
            Ok(data) => println!("{:#?}", data),
            Err(err) => println!("{:#?}", err),
        }
    }
}