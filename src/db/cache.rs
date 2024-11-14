use redis::{Client, Commands};

#[derive(Debug, Clone)]
pub struct CacheDb {
    pub client: Client,
}

impl CacheDb {
    /// create a new CacheDb
    pub async fn new(db_url: &str) -> anyhow::Result<Self, anyhow::Error> {
        let client = Client::open(db_url)?;

        Ok(CacheDb { client })
    }
    /// insert 
    pub async fn insert<T, U>(&self, key: T, val: U, exp: Option<i64>) -> Result<(), anyhow::Error> 
    where
        T: redis::ToRedisArgs,
        U: redis::ToRedisArgs,
    {
        let mut conn = self.client.get_connection()?;

        let _t = conn.set(&key, val)?;

        if let Some(exp) = exp {
            let _t = conn.expire(key, exp)?;
        }

        Ok(())
    }
    /// get value for key
    pub async fn get_val<T, U>(&self, key: T) -> Result<Option<U>, anyhow::Error> 
    where
        T: redis::ToRedisArgs,
        U: redis::FromRedisValue,
    {
        let mut connect = self.client.get_connection()?;

        let res = connect.get::<T, Option<U>>(key)?;

        Ok(res)
    }
    /// delete a key
    pub async fn del_key<T>(&self, key: T) -> Result<u32, anyhow::Error> 
    where
        T: redis::ToRedisArgs,
    {
        let mut connect = self.client.get_connection()?;

        let res = connect.del::<T, u32>(key);

        Ok(res?)
    }
    /// get keys. eg: 1* => 1, 12, 1sds
    pub async fn get_keys<T>(&self, keys: &str) -> Result<Vec<T>, anyhow::Error> 
    where
        T: redis::FromRedisValue
    {
        let mut connect = self.client.get_connection()?;

        let res = connect.keys::<&str, Vec<T>>(keys);

        Ok(res?)
    }
    /// key's val incr or decr
    pub async fn incr_or_decr<T>(&self, key: T, decr_flag: bool) -> Result<(), anyhow::Error> 
    where
        T: redis::ToRedisArgs
    {
        let mut connect = self.client.get_connection()?;

        if decr_flag {
            let _t = connect.decr(&key, 1)?;
        } else {
            let _t = connect.incr(&key, 1)?;
        }

        Ok(())
    }
    /// update key's expire
    pub async fn update_exp<T>(&self, key: T, exp: i64) -> Result<(), anyhow::Error> 
    where
        T: redis::ToRedisArgs
    {
        let mut connect = self.client.get_connection()?;
        let _t = connect.expire(key, exp)?;
        Ok(())
    }
    // insert list
    pub async fn insert_list<T, U>(&self, key: T, datas: Vec<U>, exp: Option<i64>, cover_flag: bool) -> Result<(), anyhow::Error> 
    where
        T: redis::ToRedisArgs + Clone,
        U: redis::ToRedisArgs,
    {
        let mut connect = self.client.get_connection()?;

        if cover_flag {
            let _t = connect.del::<T, ()>(key.clone())?;
        }

        for data in datas {
            let _t: () = connect.rpush(key.clone(), data)?;
        }

        if let Some(exp) = exp {
            let _t = connect.expire(key, exp)?;
        }

        Ok(())
    }
    // get list value
    pub async fn get_list<T, U>(&self, key: T) -> Result<Vec<U>, anyhow::Error> 
    where
        T: redis::ToRedisArgs,
        U: redis::FromRedisValue,
    {
        let mut connect = self.client.get_connection()?;
        let res = connect.lrange(key, 0, -1)?;
        
        Ok(res)
    }
    // update list index value
    pub async fn update_list<T, U>(&self, key: T, index: usize, status: U) -> Result<(), anyhow::Error> 
    where
        T: redis::ToRedisArgs,
        U: redis::ToRedisArgs,
    {
        let mut connect = self.client.get_connection()?;
        let _t: () = connect.lset(key, index.try_into().unwrap(), status)?;
        Ok(())
    }

    /// channel-send
    pub async fn send_msg(&self, channel: &str, msg: &str) -> Result<(), anyhow::Error> {
        let mut connect = self.client.get_connection()?;
        let _t: () = connect.publish(channel, msg)?;
        Ok(())
    }
}

// pub async fn get_keys_val_list(cache_db: CacheDb, keys: Vec<String>) -> Vec<(String, Option<Vec<String>>)> {
//     let res = futures::stream::iter(keys.into_iter()).map(|key|{
//         let cache_db = cache_db.clone();
//         async move {
//             let mut data = None;
//             if let Ok(val) = cache_db.get_val_list(&key).await {
//                 data = Some(val)
//             }
//             (key, data)
//         }
//     }).buffer_unordered(10).collect::<Vec<_>>().await;
//     res
// }
// pub async fn update_keys_val_list(cache_db: CacheDb, keys: Vec<String>, index: usize, val: &str) {
//     let _res = futures::stream::iter(keys.into_iter()).map(|key|{
//         let cache_db = cache_db.clone();
//         async move {
//             match cache_db.update_list(&key, index, val).await {
//                 Ok(_res) => {},
//                 Err(err) => {
//                     log::error!("==fail==cache_db.update_list address_chain_check err: {}", err);
//                 },
//             }
//         }
//     }).buffer_unordered(10).collect::<Vec<_>>().await;
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[async_std::test]
    async fn test_get_worker() {
        let cache_db = CacheDb::new("redis://:redis_mymm@192.168.195.71:6378")
            .await
            .unwrap();

        // let key = 1;
        // let val = 111;
        // match cache_db.insert(key, val, Some(2000)).await {
        // match cache_db.get_val::<i32, i32>(key.to_owned()).await {
        // match cache_db.del_key(key).await {
        //     Ok(data) => println!("==success==key: {key} {:?}", data),
        //     Err(err) => println!("==fail==key: {key} {}", err)
        // }

        let keys = "BTC*";
        match cache_db.get_keys::<i32>(keys).await {
            Ok(data) => println!("==success==keys: {keys} {:?}", data),
            Err(err) => println!("==fail==keys: {keys} {}", err)
        }

        // let key = "list1";
        // let val = vec![1,2,3,4];
        // match cache_db.insert_list::<&str, i32>(key, val, Some(2000), true).await {
        // match cache_db.get_list::<&str, i32>(key).await {
        // match cache_db.update_list::<&str, i32>(key, 2, 33).await {
        //     Ok(data) => println!("==success==key: {key} {:?}", data),
        //     Err(err) => println!("==fail==key: {key} {}", err)
        // }


        // match cache_db.get_val_list("TQKapDUWQgMJiwubvzKLzjbsuZ79XwF4Lr-TRX-rongtang886@gmail.com").await {


        // match cache_db.update_list("tmp", 1, "22").await {


        // let key = "a";
        // let t = cache_db.del_key(key).await;
        // println!("{:?}", t);

        // let keys = "ron-0x222-ETH";
        // match cache_db.get_val_str(keys).await {
        //     Ok(data) => {
        //         println!("{:?}", data);
        //     },
        //     Err(err) => {
        //         println!("{:?}", err.to_string());
        //     },
        // }

        // let key = "ron1@gmail.com-count";
        // let t = cache_db.update_ext(key, 10000).await;
        // println!("{:?}", t)

        // let keys = vec!["ron-0x222-ETH".to_string(), "ron-0x111-ETH".to_string()];
        // let t = get_keys_val(cache_db, keys).await;
        // println!("{:?}", t);

        // let t = cache_db.send_msg("check_channel", "0x123-ETH-ron2").await;
        // println!("{:?}", t)
    }
}
