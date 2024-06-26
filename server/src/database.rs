use sqlx::Executor;

const INIT_SCHEMA: &str = include_str!("../schema.sql");

#[derive(Debug)]
pub struct Database {
    pub pool: sqlx::Pool<sqlx::Postgres>,
}
impl Database {
    pub async fn new() -> Result<Self, sqlx::Error> {
        // Connect to the database.
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://postgres:example@localhost/database")
            .await?;

        pool.execute(INIT_SCHEMA).await?;

        Ok(Self { pool })
    }

    pub fn query(
        &self,
        _query: &str,
        _params: Vec<serde_json::Value>,
    ) -> Result<serde_json::Value, sqlx::Error> {
        todo!()
    }
}
