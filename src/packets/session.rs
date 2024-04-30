use crate::packets::header::PacketHeader;

// Details about the current session in progress
// Frequency 2 per second
// Size: 644 bytes
#[derive(Debug)]
pub struct MarshalZone {
    // Fraction (0..1) of the way through the lap marshal zone starts
    zone_start: f32,
    // -1 = invalid/unknown, 0 = none, 1 = green, 2 = blue, 3 = yellow
    zone_flag: i8,
}

#[derive(Debug)]
pub struct WeatherForecastSample {
    // 0 = unknown, 1 = P1, 2 = P2, 3 = P3, 4 = Short P, 5 = Q1
    // 6 = Q2, 7 = Q3, 8 = Short Q, 9 = OSQ, 10 = R, 11 = R2
    session_type: u8,
    // Time in minutes the forecast is for
    time_offset: u8,
    // Weather - 0 = clear, 1 = light cloud, 2 = overcast
    // 3 = light rain, 4 = heavy rain, 5 = storm
    weather: u8,
    // Track temp. in degrees Celsius
    track_temperature: i8,
    // Track temp. change – 0 = up, 1 = down, 2 = no change
    track_temp_change: i8,
    // Air temp. in degrees celsius
    air_temp: i8,
    // Air temp. change – 0 = up, 1 = down, 2 = no change
    air_temp_change: i8,
    // Rain percentage (0-100)
    rain_percentage: u8,
}

#[derive(Debug)]
pub struct PacketSessionData {
    header: PacketHeader,
    weather: u8,
    track_temperature: i8,
    air_temperature: i8,
    // Total number of laps in the session
    total_laps: u8,
    // track length in meters
    track_length: u16,
    // 0 = unknown, 1 = P1, 2 = P2, 3 = P3, 4 = Short P
    // 5 = Q1, 6 = Q2, 7 = Q3, 8 = Short Q, 9 = OSQ
    // 10 = R, 11 = R2, 12 = R3, 13 = Time Trial
    session_type: u8,
    track_id: i8, // -1 for unknown
    // Formula, 0 = F1 Modern, 1 = F1 Classic, 2 = F2,
    // 3 = F1 Generic, 4 = Beta, 5 = Supercars
    formula: u8,
    // Session time remaining (s)
    session_time_left: u16,
    // Session duration (s)
    session_duration: u16,
    // Pit speed limit km/h
    pit_speed_limit: u8,
    game_paused: u8,
    is_spectating: u8,
    spectator_car_index: u8,
    sli_pro_native_support: u8,
    // Number of marshal zones
    num_marshal_zones: u8,
    // List of marshal zones (max 21)
    marshal_zones: MarshalZone,
    // 0 = no safety car, 1 = full
    // 2 = virtual, 3 = formation lap
    safety_car_status: u8,
    network_game: u8, // 0 offline, 1 online
    num_weather_forecast_samples: u8,
    weather_forecast_samples: WeatherForecastSample,
    forecast_accuracy: u8, // 0 = Perfect, 1 = Approximate
    // Ai difficulty rating 0-110
    ai_difficulty: u8,
    season_link_identifier: u32,
    weekend_link_identifier: u32,
    session_link_identifier: u32,
    pit_stop_window_ideal_lap: u8,
    pit_stop_window_latest_lap: u8,
    // predicted rejoin after pit stop
    pit_stop_rejoin_postition: u8,
    steering_assist: u8, // 0 = off, 1 = on
    braking_assist: u8,
    gearbox_assist: u8, // 1 = manual, 2 = manual and suggested gear, 3 = auto
    pit_assist: u8,
    pit_release_assist: u8,
    ers_assist: u8,
    drs_assist: u8,
    dynamic_racing_line: u8,      // 0 = off, 1 = corners only, 2 = full
    dynamic_racing_line_type: u8, // 0 = 2D, 1 = 3D
    game_mode: u8,
    rule_set: u8,
    time_of_day: u32, // Local time of day (minutes since midnight)
    // 0 = None, 2 = Very Short, 3 = Short, 4 = Medium
    // 5 = Medium Long, 6 = Long, 7 = Full
    session_length: u8,
    speed_units_lead_player: u8, // 0 = MPH, 1 = KPH
    temp_units_lead_player: u8,  // 0 = C, 1 = F
    speed_units_secondary_player: u8,
    temp_units_secondary_player: u8,
    num_safety_car_periods: u8,
    num_red_flag_periods: u8,
}
