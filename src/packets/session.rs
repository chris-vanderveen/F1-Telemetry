use crate::packets::header::PacketHeader;
use byteorder::{ByteOrder, LittleEndian};
use serde::Serialize;

// Details about the current session in progress
// Frequency 2 per second
// Size: 644 bytes

// 5 bytes * 21 = 105 bytes
#[derive(Debug, Serialize)]
pub struct MarshalZone {
    // Fraction (0..1) of the way through the lap marshal zone starts
    zone_start: f32,
    // -1 = invalid/unknown, 0 = none, 1 = green, 2 = blue, 3 = yellow
    zone_flag: i8,
}

// 8 bytes * 56 = 448 bytes
#[derive(Debug, Serialize)]
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

#[derive(Debug, Serialize)]
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
    marshal_zones: Vec<MarshalZone>,
    // 0 = no safety car, 1 = full
    // 2 = virtual, 3 = formation lap
    safety_car_status: u8,
    network_game: u8, // 0 offline, 1 online
    num_weather_forecast_samples: u8,
    weather_forecast_samples: Vec<WeatherForecastSample>,
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

impl PacketSessionData {
    pub fn from_bytes(data: &[u8]) -> Self {
        let header = PacketHeader::from_bytes(&data[0..29]);

        // Loop through all of the MarshalZones and put in a Vec. Each Zone is 5 bytes
        let num_marshal_zones = data[47] as usize;
        let mut marshal_zones = Vec::new();
        let mut offset = 48;
        for i in 0..num_marshal_zones {
            let zone = MarshalZone::from_bytes(&data[offset..offset + 5]);
            match zone {
                Ok(zone) => {
                    marshal_zones.push(zone);
                }
                Err(e) => {
                    eprintln!("Error parsing MarshalZone: {:?}", e);
                    break;
                }
            }
            offset += 5;
        }

        // Loop through all of the WeatherForecastSamples and put in a Vec. Each sample is 8 bytes.
        let num_weather_forecast_samples = data[155] as usize;
        let mut weather_forecast_samples = Vec::new();
        let mut offset = 156;
        for i in 0..num_weather_forecast_samples {
            let sample = WeatherForecastSample::from_bytes(&data[offset..offset + 8]);
            match sample {
                Ok(sample) => {
                    weather_forecast_samples.push(sample);
                }
                Err(e) => {
                    eprintln!("Error parsing WeatherForecastSample: {:?}", e);
                    break;
                }
            }
            offset += 8;
        }

        PacketSessionData {
            header: header,
            weather: data[29],
            track_temperature: data[30] as i8,
            air_temperature: data[31] as i8,
            total_laps: data[32],
            track_length: LittleEndian::read_u16(&data[33..35]),
            session_type: data[35],
            track_id: data[36] as i8,
            formula: data[37],
            session_time_left: LittleEndian::read_u16(&data[38..40]),
            session_duration: LittleEndian::read_u16(&data[40..42]),
            pit_speed_limit: data[42],
            game_paused: data[43],
            is_spectating: data[44],
            spectator_car_index: data[45],
            sli_pro_native_support: data[46],
            num_marshal_zones: data[47],
            marshal_zones: marshal_zones,
            safety_car_status: data[153],
            network_game: data[154],
            num_weather_forecast_samples: data[155],
            weather_forecast_samples: weather_forecast_samples,
            forecast_accuracy: data[604],
            ai_difficulty: data[605],
            season_link_identifier: LittleEndian::read_u32(&data[606..610]),
            weekend_link_identifier: LittleEndian::read_u32(&data[610..614]),
            session_link_identifier: LittleEndian::read_u32(&data[614..618]),
            pit_stop_window_ideal_lap: data[618],
            pit_stop_window_latest_lap: data[619],
            pit_stop_rejoin_postition: data[620],
            steering_assist: data[621],
            braking_assist: data[622],
            gearbox_assist: data[623],
            pit_assist: data[624],
            pit_release_assist: data[625],
            ers_assist: data[626],
            drs_assist: data[627],
            dynamic_racing_line: data[628],
            dynamic_racing_line_type: data[629],
            game_mode: data[630],
            rule_set: data[631],
            time_of_day: LittleEndian::read_u32(&data[632..636]),
            session_length: data[636],
            speed_units_lead_player: data[637],
            temp_units_lead_player: data[638],
            speed_units_secondary_player: data[640],
            temp_units_secondary_player: data[641],
            num_safety_car_periods: data[642],
            num_red_flag_periods: data[643],
        }
    }
}

impl MarshalZone {
    fn from_bytes(data: &[u8]) -> Result<Self, std::io::Error> {
        if data.len() < 5 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid data",
            ));
        }
        let bytes: [u8; 4] = data[0..4].try_into().map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Failed to convert slice to array",
            )
        })?;

        let zone_start = LittleEndian::read_f32(&data[0..4]);
        let zone_flag = data[4] as i8;

        Ok(MarshalZone {
            zone_start,
            zone_flag,
        })
    }
}

impl WeatherForecastSample {
    fn from_bytes(data: &[u8]) -> Result<Self, std::io::Error> {
        if data.len() < 8 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid Data",
            ));
        }

        Ok(WeatherForecastSample {
            session_type: data[0],
            time_offset: data[1],
            weather: data[2],
            track_temperature: data[3] as i8,
            track_temp_change: data[4] as i8,
            air_temp: data[5] as i8,
            air_temp_change: data[6] as i8,
            rain_percentage: data[7],
        })
    }
}
