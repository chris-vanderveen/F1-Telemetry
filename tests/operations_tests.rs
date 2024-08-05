#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use project::db::operations::get_user;
    use std::env;
    use tokio_postgres::{Client, NoTls};

    async fn setup_test_db() -> Client {
        dotenv().ok();
        env_logger::init();

        // Connect to Postgres DB
        let postgres_config = env::var("APP_DATABASE_URL").unwrap();
        let (client, connection) = tokio_postgres::connect(&postgres_config, NoTls)
            .await
            .expect("Failed to connect to db");

        tokio::spawn(connection);

        client
    }

    #[tokio::test]
    async fn test_get_user() {
        let client = setup_test_db().await;

        let test_username = "testuser";
        let insert_query = "INSERT INTO users (username) VALUES ($1) RETURNING id";
        let test_user_id: i32 = client
            .query_one(insert_query, &[&test_username])
            .await
            .expect("Failed to insert test user")
            .get(0);

        let result = get_user(&client, test_user_id).await;

        match result {
            Ok(Some(user)) => {
                assert_eq!(user.id, test_user_id);
                assert_eq!(user.username, test_username);
            }
            _ => panic!("Failed to retrieve test user"),
        }
    }
}
