use crate::packets::header::PacketHeader;
use byteorder::{ByteOrder, LittleEndian};
use serde::{Deserialize, Serialize};

// Frequency: 20/s (cycles through all 20 cars once per second)
// Size: 14 bytes * 100 = 1400 bytes
#[derive(Debug, Serialize, Deserialize)]
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

// Size: 3 bytes * 8 = 24 bytes
#[derive(Debug, Serialize, Deserialize)]
pub struct TyreStintHistoryData {
    end_lap: u8, // Lap the tyre usage ends on
    tyre_actual_compound: u8,
    tyre_visual_compound: u8,
}

// Size: 1460 bytes
#[derive(Debug, Serialize, Deserialize)]
pub struct PacketSessionHistoryData {
    pub header: PacketHeader,
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

impl PacketSessionHistoryData {
    pub fn from_bytes(data: &[u8]) -> Self {
        let header = PacketHeader::from_bytes(&data[0..29]);
        let num_laps = data[30];
        let num_tyre_stints = data[31];
        let mut offset = 36;
        let mut lap_history_data = Vec::new();
        let mut tyre_stint_history_data = Vec::new();

        for _i in 0..num_laps {
            let lap = LapHistoryData::from_bytes(&data[offset..offset + 14]);
            lap_history_data.push(lap);
            offset += 14;
        }

        offset = 1435;

        for _i in 0..num_tyre_stints {
            let stint = TyreStintHistoryData::from_bytes(&data[offset..offset + 3]);
            tyre_stint_history_data.push(stint);
            offset += 3;
        }

        PacketSessionHistoryData {
            header: header,
            car_idx: data[29],
            num_laps: num_laps,
            num_tyre_stints: num_tyre_stints,
            best_lap_time_num: data[32],
            best_sector1_lap_num: data[33],
            best_sector2_lap_num: data[34],
            best_sector3_lap_num: data[35],
            lap_history_data: lap_history_data,
            tyre_stint_history_data: tyre_stint_history_data,
        }
    }
}

impl TyreStintHistoryData {
    fn from_bytes(data: &[u8]) -> Self {
        TyreStintHistoryData {
            end_lap: data[0],
            tyre_actual_compound: data[1],
            tyre_visual_compound: data[2],
        }
    }
}

impl LapHistoryData {
    fn from_bytes(data: &[u8]) -> Self {
        LapHistoryData {
            lap_time_ms: LittleEndian::read_u32(&data[0..4]),
            sector_1_time_ms: LittleEndian::read_u16(&data[4..6]),
            sector_1_time_mins: data[6],
            sector_2_time_ms: LittleEndian::read_u16(&data[7..9]),
            sector_2_time_mins: data[9],
            sector_3_time_ms: LittleEndian::read_u16(&data[10..12]),
            sector_3_time_mins: data[12],
            lap_valid_bit_flags: data[13],
        }
    }
}
