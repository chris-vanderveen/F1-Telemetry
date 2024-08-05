use crate::db::models::User;
use tokio_postgres::{Client, Error};

pub async fn get_user(client: &Client, user_id: i32) -> Result<Option<User>, Error> {
    let row = client
        .query_opt("SELECT id, username FROM users WHERE id = $1", &[&user_id])
        .await?;

    Ok(row.map(|row| User {
        id: row.get(0),
        username: row.get(1),
    }))
}
