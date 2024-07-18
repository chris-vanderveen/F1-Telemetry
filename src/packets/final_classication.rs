use crate::packets::header::PacketHeader;
use byteorder::{ByteOrder, LittleEndian};
use serde::{Deserialize, Serialize};

// Once at the end of race
// Size: 41 bytes
#[derive(Debug, Serialize, Deserialize)]
pub struct FinalClassificationData {
    position: u8,      // Finishing pos
    num_laps: u8,      // NUmber of laps completed
    grid_position: u8, // Grid position of car
    points: u8,        // number of points scored
    num_pit_stops: u8,
    results_status: u8,   // 0 = invalid, 1 = inactive, 2 = active
    best_lap_time: u32,   // (Ms)
    total_race_time: u32, // (ms)
    penalties_time: u8,   // Accumulation of penalty time (s)
    num_penalties: u8,
    num_tyre_stints: u8,
    tyre_stints_actual: Vec<u8>,
    tyre_stints_visual: Vec<u8>,
    tyre_stints_end_lap: Vec<u8>,
}

// Size: 1020 bytes
#[derive(Debug, Serialize, Deserialize)]
pub struct PacketFinalClassification {
    pub header: PacketHeader,
    num_cars: u8,
    final_classification_data: Vec<FinalClassificationData>,
}

impl PacketFinalClassification {
    pub fn from_bytes(data: &[u8]) -> Self {
        let header = PacketHeader::from_bytes(&data[0..29]);
        let num_cars = data[29];
        let mut final_classification_data = Vec::new();
        let mut offset = 30;

        for _i in 0..num_cars {
            let final_data = FinalClassificationData::from_bytes(&data[offset..offset + 41]);
            final_classification_data.push(final_data);
            offset += 41;
        }

        PacketFinalClassification {
            header: header,
            num_cars: num_cars,
            final_classification_data: final_classification_data,
        }
    }
}

impl FinalClassificationData {
    fn from_bytes(data: &[u8]) -> Self {
        let mut tyre_stints_actual = Vec::new();
        let mut tyre_stints_visual = Vec::new();
        let mut tyre_stints_end_lap = Vec::new();
        let offset = 17;

        for i in 0..7 {
            tyre_stints_actual.push(data[offset + i]);
            tyre_stints_visual.push(data[offset + i + 8]);
            tyre_stints_end_lap.push(data[offset + i + 16]);
        }

        FinalClassificationData {
            position: data[0],
            num_laps: data[1],
            grid_position: data[2],
            points: data[3],
            num_pit_stops: data[4],
            results_status: data[5],
            best_lap_time: LittleEndian::read_u32(&data[6..10]),
            total_race_time: LittleEndian::read_u32(&data[10..14]),
            penalties_time: data[14],
            num_penalties: data[15],
            num_tyre_stints: data[16],
            tyre_stints_actual: tyre_stints_actual,
            tyre_stints_visual: tyre_stints_visual,
            tyre_stints_end_lap: tyre_stints_end_lap,
        }
    }
}
