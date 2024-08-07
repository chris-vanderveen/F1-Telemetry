use crate::db::models::User;
use tokio_postgres::{Client, Error};

use super::models::Session;

pub async fn get_user(client: &Client, user_id: i32) -> Result<Option<User>, Error> {
    let row = client
        .query_opt("SELECT id, username FROM users WHERE id = $1", &[&user_id])
        .await?;

    Ok(row.map(|row| User {
        id: row.get(0),
        username: row.get(1),
    }))
}

pub async fn add_user(client: &Client, user: &User) -> Result<(), Error> {
    let query = "INSERT INTO users (id, username) VALUES ($1, $2)";
    client.execute(query, &[&user.id, &user.username]).await?;
    Ok(())
}

pub async fn delete_user(client: &Client, id: i32) -> Result<(), Error> {
    let query = "DELETE FROM users WHERE id = $1";
    client.execute(query, &[&id]).await?;
    Ok(())
}

pub async fn add_session(client: &Client, session: &Session) -> Result<(), Error> {
    const QUERY: &str = "
        INSERT INTO sessions (
            user_id, session_name, session_start, session_end, session_type,
            session_status, forecast_accuracy, track_id, total_laps, track_length,
            pit_speed_limit, session_duration, num_marshal_zones, network_game,
            ai_difficulty, steering_assist, braking_assist, gearbox_assist,
            pit_assist, pit_release_assist, ers_assist, drs_assist, game_mode,
            rule_set, time_of_day, session_length, speed_units, temperature_units
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15,
            $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28
        )";

    client
        .execute(
            QUERY,
            &[
                &session.user_id,
                &session.session_name,
                &session.session_start,
                &session.session_end,
                &session.session_type,
                &session.session_status,
                &session.forecast_accuracy,
                &session.track_id,
                &session.total_laps,
                &session.track_length,
                &session.pit_speed_limit,
                &session.session_duration,
                &session.num_marshal_zones,
                &session.network_game,
                &session.ai_difficulty,
                &session.steering_assist,
                &session.braking_assist,
                &session.gearbox_assist,
                &session.pit_assist,
                &session.pit_release_assist,
                &session.ers_assist,
                &session.drs_assist,
                &session.game_mode,
                &session.rule_set,
                &session.time_of_day,
                &session.session_length,
                &session.speed_units,
                &session.temperature_units,
            ],
        )
        .await?;

    Ok(())
}
