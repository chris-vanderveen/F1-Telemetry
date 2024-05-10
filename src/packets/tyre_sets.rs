use crate::packets::header::PacketHeader;
use byteorder::{ByteOrder, LittleEndian};
use serde::{Deserialize, Serialize};

// Frequency: 20/s cycles through cars
// Size: 10 bytes * 20 = 200 bytes
#[derive(Debug, Serialize, Deserialize)]
pub struct TyreSetData {
    actual_tyre_compound: u8,
    visual_tyre_compound: u8,
    wear: u8, // Tyre wear (Percentage)
    available: u8,
    recommended_session: u8,
    life_span: u8,       // laps left in this tyre set
    usable_life: u8,     // Max number of recommended laps
    lap_delta_time: i16, // delta lap time (ms) compared to currently fitted tyre
    fitted: u8,          // Whether the set is fitted or not
}

// Size: 231 bytes
#[derive(Debug, Serialize, Deserialize)]
pub struct PacketTyreSetData {
    pub header: PacketHeader,
    car_idx: u8,
    tyre_set_data: Vec<TyreSetData>, // 20 total, 13 dry + 7 wet
    fitted_idx: u8,                  // Index into vec of fitted tyre
}

impl PacketTyreSetData {
    pub fn from_bytes(data: &[u8]) -> Self {
        let header = PacketHeader::from_bytes(&data[0..29]);
        let mut offset = 30;
        let mut tyre_set_data = Vec::new();

        for _i in 0..19 {
            let tyre_set = TyreSetData::from_bytes(&data[offset..offset + 10]);
            tyre_set_data.push(tyre_set);
            offset += 10;
        }

        PacketTyreSetData {
            header: header,
            car_idx: data[29],
            tyre_set_data: tyre_set_data,
            fitted_idx: data[230],
        }
    }
}

impl TyreSetData {
    fn from_bytes(data: &[u8]) -> Self {
        TyreSetData {
            actual_tyre_compound: data[0],
            visual_tyre_compound: data[1],
            wear: data[2],
            available: data[3],
            recommended_session: data[4],
            life_span: data[5],
            usable_life: data[6],
            lap_delta_time: LittleEndian::read_i16(&data[7..9]),
            fitted: data[9],
        }
    }
}
