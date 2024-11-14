use std::time::Duration;

/// 重试函数 test_count:大尝试次数(None则无限重试) sleep_time:错误后休眠时间(None则不休眠)
async fn retry_fn<T, F, Fut>(
    test_count: Option<usize>,
    sleep_time: Option<Duration>,
    f: F,
) -> Result<T, anyhow::Error>
where
    F: Fn() -> Fut,
    Fut: futures::Future<Output = Result<T, anyhow::Error>>,
{
    let mut count = 1;
    loop {
        match f().await {
            Ok(result) => return Ok(result),
            Err(err) => {
                println!("retry_fn test_count {} sleep {:?} failed: {}. ", count, sleep_time, err);
                // 达到最大尝试次数, 返回最后一次错误
                if test_count.is_some_and(|test_count|test_count <= count) {
                    return Err(err.into());
                }
                // 休眠
                if let Some(sleep_time) = sleep_time {
                    async_std::task::sleep(sleep_time).await;
                }
            }
        }
        count += 1;
    }
}

// 限制异步函数超时函数
pub async fn with_timeout<F, T>(timeout_duration: Duration, fut: F) -> Result<T, anyhow::Error>
where
    F: std::future::Future<Output = Result<T, anyhow::Error>>,
{
    async_std::future::timeout(timeout_duration, fut).await?
}

// 示例异步函数
async fn _example_function(param: i32) -> Result<i32, anyhow::Error> {
    for i in 0..param {
        println!("--sleep 1s {}", i + 1);
        async_std::task::sleep(Duration::from_secs(1)).await;
    }
    if param > 0 {
        Ok(1)
    } else {
        Err(anyhow::anyhow!("error-msg".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use super::*;

    #[async_std::test]
    async fn tete() {
        // evm api
        let timeout = Some(Duration::from_secs(1));
        let base_url = "http://192.168.195.239:50545/jsonrpc";
        let ok_client = crate::utils::self_client::SelfClient::new(base_url, vec![], timeout).unwrap();

        let val = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "eth_getBlockByNumber",
            "params": [
                "0xaaa1b4",
                false
            ],
            "id": 1
        });

        let f = || {
            let task = ok_client.clone().http_post::<Value, Value>(None, val.clone());
            with_timeout(Duration::from_secs(1), task)
        };

        match retry_fn::<Value, _, _>(
            Some(1), 
            Some(Duration::from_secs(1)),
            f
        ).await {
            Ok(data) => {
                println!("{:#?}", data)
            },
            Err(err) => {
                println!("{:#?}", err)
            },
        }
    }
}
