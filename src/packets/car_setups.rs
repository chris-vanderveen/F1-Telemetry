use crate::packets::header::PacketHeader;
use byteorder::{ByteOrder, LittleEndian};
use serde::{Deserialize, Serialize};

// Frequency: 2/s

// Size: 49 bytes
#[derive(Debug, Serialize, Deserialize)]
pub struct CarSetupData {
    front_wing: u8,    // Front wing aero
    rear_wing: u8,     // Rear wing aero
    on_throttle: u8,   // Diff adjustment on throttle (Percentage)
    off_throttle: u8,  // Diff adjustment off throttle
    front_camber: f32, // Front camber angle
    rear_camber: f32,  // Rear camber angle
    front_toe: f32,    // Front toe angle
    rear_toe: f32,     // Rear toe angle
    front_suspension: u8,
    rear_suspension: u8,
    front_anti_roll_bar: u8,
    rear_anti_roll_bar: u8,
    front_suspension_height: u8,
    rear_suspension_height: u8,
    brake_pressure: u8,    // Brake pressure (Percentage)
    brake_bias: u8,        // Brake bias (Percentage)
    rl_tyre_pressure: f32, // Rear left tyre pressure (PSI)
    rr_tyre_pressure: f32,
    fl_tyre_pressure: f32,
    fr_tyre_pressure: f32,
    ballast: u8,
    fuel_load: f32,
}

// Size: 1107 bytes
#[derive(Debug, Serialize, Deserialize)]
pub struct PacketCarSetupData {
    pub header: PacketHeader,
    car_setups: Vec<CarSetupData>,
}

impl PacketCarSetupData {
    pub fn from_bytes(data: &[u8]) -> Self {
        let header = PacketHeader::from_bytes(&data[0..29]);
        let mut car_setup_data = Vec::new();
        let mut offset = 29;

        for _i in 0..22 {
            let car_data = CarSetupData::from_bytes(&data[offset..offset + 49]);
            car_setup_data.push(car_data);
            offset += 49;
        }

        PacketCarSetupData {
            header: header,
            car_setups: car_setup_data,
        }
    }
}

impl CarSetupData {
    fn from_bytes(data: &[u8]) -> Self {
        CarSetupData {
            front_wing: data[0],
            rear_wing: data[1],
            on_throttle: data[2],
            off_throttle: data[3],
            front_camber: LittleEndian::read_f32(&data[4..8]),
            rear_camber: LittleEndian::read_f32(&data[8..12]),
            front_toe: LittleEndian::read_f32(&data[12..16]),
            rear_toe: LittleEndian::read_f32(&data[16..20]),
            front_suspension: data[20],
            rear_suspension: data[21],
            front_anti_roll_bar: data[22],
            rear_anti_roll_bar: data[23],
            front_suspension_height: data[24],
            rear_suspension_height: data[25],
            brake_pressure: data[26],
            brake_bias: data[27],
            rl_tyre_pressure: LittleEndian::read_f32(&data[28..32]),
            rr_tyre_pressure: LittleEndian::read_f32(&data[32..36]),
            fl_tyre_pressure: LittleEndian::read_f32(&data[36..40]),
            fr_tyre_pressure: LittleEndian::read_f32(&data[40..44]),
            ballast: data[44],
            fuel_load: LittleEndian::read_f32(&data[45..49]),
        }
    }
}
