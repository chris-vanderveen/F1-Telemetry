#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc};
    use dotenv::dotenv;
    use project::db::models::Session;
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

    #[tokio::test]
    pub async fn test_add_session() {}

    // Helper function to create a dummy Session struct
    pub fn create_dummy_session() -> Session {
        Session {
            user_id: 1,
            session_name: String::from("Test Race"),
            session_start: Utc::now().timestamp(),
            session_end: Utc::now().timestamp() + 3600, // 1 hour later
            session_type: String::from("Race"),
            session_status: 1, // 1 means "completed"
            forecast_accuracy: 80,
            track_id: 5,
            total_laps: 50,
            track_length: 5000, // 5000 meters
            pit_speed_limit: 60,
            session_duration: 3600, // 1 hour in seconds
            num_marshal_zones: 20,
            network_game: 0, // 0 means "no"
            ai_difficulty: 90,
            steering_assist: 0, // 0 means "off"
            braking_assist: 0,
            gearbox_assist: 0,
            pit_assist: 0,
            pit_release_assist: 0,
            ers_assist: 0,
            drs_assist: 0,
            game_mode: 3,         // Arbitrary value
            rule_set: 1,          // Arbitrary value
            time_of_day: 14,      // represents 2 PM
            session_length: 100,  // Arbitrary value
            speed_units: 0,       // 0 means "km/h"
            temperature_units: 0, // 0 means "Celsius"
        }
    }
}
