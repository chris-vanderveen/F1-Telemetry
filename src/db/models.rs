#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
}

#[derive(Debug)]
pub struct Session {
    pub user_id: i64,
    pub session_name: String,
    pub session_start: i64,
    pub session_end: i64,
    pub session_type: String,
    pub session_status: i8,
    pub forecast_accuracy: i8,
    pub track_id: i8,
    pub total_laps: i8,
    pub track_length: i32,
    pub pit_speed_limit: i8,
    pub session_duration: i32,
    pub num_marshal_zones: i8,
    pub network_game: i8,
    pub ai_difficulty: i8,
    pub steering_assist: i8,
    pub braking_assist: i8,
    pub gearbox_assist: i8,
    pub pit_assist: i8,
    pub pit_release_assist: i8,
    pub ers_assist: i8,
    pub drs_assist: i8,
    pub game_mode: i8,
    pub rule_set: i8,
    pub time_of_day: i8,
    pub session_length: i8,
    pub speed_units: i8,
    pub temperature_units: i8,
}
