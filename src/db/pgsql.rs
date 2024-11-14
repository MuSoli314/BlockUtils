use sqlx::{pool::PoolOptions, postgres::PgConnectOptions, ConnectOptions, Pool, Postgres};
use std::str::FromStr;

#[derive(Clone)]
pub struct Db {
    pub pool: Pool<Postgres>,
}

impl Db {
    pub async fn new(
        db_url: &str,
        min_connections: u32,
        max_connections: u32,
    ) -> Result<Self, anyhow::Error> {
        let options = PgConnectOptions::from_str(db_url)?;

        let pool = PoolOptions::new()
            .min_connections(min_connections)
            .max_connections(max_connections)
            .connect_with(options.disable_statement_logging())
            .await?;

        Ok(Self { pool })
    }

    pub async fn execute_sql(
        &self,
        sql: String,
        params: Vec<String>,
    ) -> Result<u64, anyhow::Error> {
        let mut query = sqlx::query(&sql);

        for i in params {
            query = query.bind(i)
        }

        let res = query.execute(&self.pool).await;

        Ok(res?.rows_affected())
    }

    pub async fn select_one<T>(self, sql: String, params: Vec<String>) -> Result<T, anyhow::Error>
    where
        T: for<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> + Send + Sync + Unpin,
    {
        let mut query = sqlx::query_as::<_, T>(&sql);

        for i in params {
            query = query.bind(i)
        }

        let res = query.fetch_one(&self.pool).await;

        Ok(res?)
    }

    pub async fn select_all<T>(
        self,
        sql: String,
        params: Vec<String>,
    ) -> Result<Vec<T>, anyhow::Error>
    where
        T: for<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> + Send + Sync + Unpin,
    {
        let mut query = sqlx::query_as::<_, T>(&sql);

        for i in params {
            query = query.bind(i)
        }

        let res = query.fetch_all(&self.pool).await;

        Ok(res?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[async_std::test]
    async fn test_select_() {
        let url = "postgresql://user:password@host:port/DB_NAME";

        match Db::new(url, 1, 5).await {
            Ok(db) => {
                // let sql = "SELECT 'name' as name, 13 as age".to_owned();
                let sql = "SELECT 123".to_owned();
                let params = Vec::new();

                match db.select_one::<(i32,)>(sql, params).await {
                    Ok(data) => {
                        println!("{:#?}", data);
                    }
                    Err(err) => println!("{}", err),
                }
            }
            Err(err) => println!("{}", err),
        }
    }
}
