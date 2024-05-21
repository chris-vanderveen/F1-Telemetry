use tokio_postgres::{Client, Error, NoTls};

pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn new() -> Option<Self> {
        Some(Self {
            database_url: std::env::var("DATABASE_URL").ok()?,
        })
    }
}

pub struct Db {
    pub client: Client,
}

impl Db {
    pub async fn new(conn_str: &str) -> Result<Self, Error> {
        let (client, connection) = tokio_postgres::connect(conn_str, NoTls).await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Connection error: {}", e);
            }
        });
        Ok(Db { client })
    }
}
