use crate::packets::header::PacketHeader;

#[derive(Debug)]
pub struct LapData {
    last_lap_time_ms: u32,
    current_lap_time_ms: u32,
    sector_1_time_ms: u16,
    sector_1_time_mins: u8,
    sector_2_time_ms: u16,
    sector_2_time_mins: u8,
    delta_to_car_ahead_ms: u16,
    delta_to_race_leader: u16,
    lap_distance: f32,   // current distance in meters (vehicle)
    total_distance: f32, // Total distance travelled in session
    safety_car_delta: f32,
    car_postion: u8,
    current_lap_number: u8,
    pit_status: u8,              // 0 = none, 1 = pitting, 2 = in pit area
    num_pit_stops: u8,           // Number of pit stops taken in this race
    sector: u8,                  // 1, 2, 3
    current_lap_invalid: u8,     // 0 = valid, 1 = invalid
    penalties: u8,               // Accumulated time penalties in (s)
    total_warnings: u8,          // Total number of warnings issued
    corner_cutting_warnings: u8, // Accumulated corner-cut warnings
    num_unserved_drive_through_pens: u8,
    num_unserved_stop_go_pens: u8,
    grid_position: u8, // Grid pos race was started in
    // Status of driver - 0 = in garage, 1 = flying lap
    // 2 = in lap, 3 = out lap, 4 = on track
    driver_status: u8,
    // Result status - 0 = invalid, 1 = inactive, 2 = active
    // 3 = finished, 4 = didnotfinish, 5 = disqualified
    // 6 = not classified, 7 = retired
    result_status: u8,
    pit_lane_timer_active: u8, // 0 = inactive, 1 = active
    pit_lane_time_in_lane_ms: u16,
    pit_stop_time_ms: u16, // Time of actual pit stop in ms
    pit_stop_should_serve_pen: u8,
}

#[derive(Debug)]
pub struct PacketLapData {
    header: PacketHeader,
    lap_data: LapData,
    time_trial_pb_car_idx: u8, // index of pb car in time trial (255 if invalid)
    time_trial_rival_car_idx: u8,
}
