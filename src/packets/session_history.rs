use crate::packets::header::PacketHeader;

// Frequency: 20/s (cycles through all 20 cars once per second)
// Size: 1460 bytes
#[derive(Debug)]
pub struct LapHistoryData {
    lap_time_ms: u32,
    sector_1_time_ms: u16,
    sector_1_time_mins: u8,
    sector_2_time_ms: u16,
    sector_2_time_mins: u8,
    sector_3_time_ms: u16,
    sector_3_time_mins: u8,
    lap_valid_bit_flags: u8, // 0x01 = lap valid, 0x02 = sector 1 valid, 0x04 = sector 2 valid, 0x08 = sector 3 valid
}

#[derive(Debug)]
pub struct TyreStintHistoryData {
    end_lap: u8, // Lap the tyre usage ends on
    tyre_actual_compound: u8,
    tyre_visual_compound: u8,
}

#[derive(Debug)]
pub struct PacketSessionHistoryData {
    header: PacketHeader,
    car_idx: u8,           // Index of the car that this data relates to
    num_laps: u8,          // number of laps in the data (including current)
    num_tyre_stints: u8,   // Number of tyre stints in data
    best_lap_time_num: u8, // Lap number best lap was achieved
    best_sector1_lap_num: u8,
    best_sector2_lap_num: u8,
    best_sector3_lap_num: u8,
    lap_history_data: Vec<LapHistoryData>, // Vector of lap data structs, max of 100 laps
    tyre_stint_history_data: Vec<TyreStintHistoryData>, // Max vec size = 8
}
