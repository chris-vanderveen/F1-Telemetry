#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use project::db::operations::get_user;
    use tokio_postgres::{Client, NoTls};

    async fn setup_test_db() -> Client {
        dotenv().ok();
        env_logger::init();

        // Connect to Postgres DB
        let postgres_config = "host=localhost dbname=f1_data user=admin";
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
        let test_password = "testPassword";
        let insert_query = "INSERT INTO users (username, password) VALUES ($1, $2) RETURNING id";
        let test_user_id: i32 = client
            .query_one(insert_query, &[&test_username, &test_password])
            .await
            .expect("Failed to insert test user")
            .get(0);

        let result = get_user(&client, test_user_id).await;

        match result {
            Ok(Some(user)) => {
                assert_eq!(user.id, test_user_id);
                assert_eq!(user.username, test_username);
            }
            _ => println!("Failed to retrieve test user"),
        }

        let delete_query = "DELETE FROM users WHERE id = $1";
        client
            .execute(delete_query, &[&test_user_id])
            .await
            .expect("Failed to delete test user");

        let reset_serial_query = "ALTER SEQUENCE users_id_seq RESTART WITH 1";
        client
            .execute(reset_serial_query, &[])
            .await
            .expect("Failed to reset serial counter");
    }

    #[tokio::test]
    async fn test_add_users_x100() {
        let client = setup_test_db().await;
        let test_password = "password";
        let insert_query = "INSERT INTO users (username, password) VALUES ($1, $2) RETURNING id";

        for i in 0..100 {
            let test_username = format!("name{}", i);

            let test_user_id: i32 = client
                .query_one(insert_query, &[&test_username, &test_password])
                .await
                .expect("Failed to insert test user")
                .get(0);

            let result = get_user(&client, test_user_id).await;

            match result {
                Ok(Some(user)) => {
                    assert_eq!(user.id, test_user_id);
                    assert_eq!(user.username, test_username);
                }
                _ => println!("Failed to retrieve user"),
            }
            if i % 5 == 0 {
                println!("{} users inserted", i);
            }
        }
    }

    #[tokio::test]
    pub async fn delete_users_x100() {
        let client = setup_test_db().await;
        let delete_query = "DELETE FROM users WHERE id = $1";
        let reset_serial_query = "ALTER SEQUENCE users_id_seq RESTART WITH 1";

        for i in 1..=100 {
            client
                .execute(delete_query, &[&(i as i32)])
                .await
                .expect("Failed to delete user");
        }

        client
            .execute(reset_serial_query, &[])
            .await
            .expect("Failed to reset serial counter");
    }
}
