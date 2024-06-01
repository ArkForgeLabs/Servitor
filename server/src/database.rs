const DATABASE_INIT: &str = r#"
CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, username TEXT, email TEXT, password TEXT);
CREATE TABLE IF NOT EXISTS graphs (id INTEGER PRIMARY KEY, graph_name TEXT, user INTEGER NOT NULL, nodes: TEXT);
"#;

#[derive(Debug)]
pub struct Database {
    pub pool: sqlx::Pool<sqlx::Postgres>,
}
impl Database {
    pub async fn new() -> Result<Self, sqlx::Error> {
        // Connect to the database.
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://testuser:example@localhost/testdb")
            .await?;

        Ok(Self { pool })
    }

    pub fn query(
        &self,
        query: &str,
        params: Vec<serde_json::Value>,
    ) -> Result<serde_json::Value, sqlx::Error> {
        todo!()
    }
}
