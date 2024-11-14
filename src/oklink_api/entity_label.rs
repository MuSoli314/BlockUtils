use super::*;

#[derive(Debug, Clone, Deserialize)]
pub struct EntityLabelData {
    pub label: String,
    pub address: String,
}

impl OkClient {
    pub async fn get_entity_label(
        &self,
        chain: &str,
        addrs: Vec<String>,
    ) -> Result<OkApiData<Vec<EntityLabelData>>, anyhow::Error> {
        let addrs_str = addrs.join(",");

        let join_url = format!(
            "address/entity-label?chainShortName={}&address={}",
            chain, addrs_str
        );
        // println!("{}", url);

        let res = self.get(&join_url).await?;

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[async_std::test]
    async fn test_() {
        let ok_client = init_ok_client();

        let chain = "tron";
        let addrs = vec![
            "TMuA6YqfCeX8EhbfYEg5y7S4DqzSJireY9".to_string(),
            "TPsjfDqBBXoMfHSbUWMqMAW9RktbGrCyCH".to_string(),
            "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t".to_string(), // USDT
        ];
        match ok_client.get_entity_label(chain, addrs).await {
            Ok(data) => {
                println!("{:#?}", data);
            }
            Err(err) => {
                println!("{:?}", err)
            }
        }
    }
}
