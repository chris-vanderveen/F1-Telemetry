use crate::packets::header::PacketHeader;
use byteorder::{ByteOrder, LittleEndian};
use serde::{Deserialize, Serialize};

// Frequency: Rate as specified in menu
// Size: 53 bytes
#[derive(Debug, Serialize, Deserialize)]
pub struct CarStatusData {
    traction_control: u8,   // 0 = off, 1 = medium, 2 = full
    anti_lock_brakes: u8,   // 0 = off, 1 = on
    fuel_mix: u8,           // 0 = lean, 1 = standard, 2 = rich, 3 = max
    front_brake_bias: u8,   // (Percentage)
    pit_limiter_status: u8, // 0 = off, 1 = on
    fuel_in_tank: f32,      // Current fuel mass
    fuel_capacity: f32,
    fuel_remaining_laps: f32, // Fuel remaining in terms of laps
    max_rpm: u16,             // Cars max rpm
    idle_rpm: u16,
    max_gears: u8,
    drs_allowed: u8,              // 0 = false, 1 = true
    drs_activation_distance: u16, // How far until drs zone
    actual_tyre_compund: u8,
    visual_tyre_compund: u8,
    tyres_age_laps: u8,
    vehicle_fia_flags: i8, // -1 = invalid, 0 = none, 1 = green, 2 = blue, 3 = yellow
    engine_power_ice: f32, // (W)
    engine_power_mguk: f32, // (W)
    ers_store_energy: f32, // ERS energy store in Joules
    ers_deploy_mode: u8,   // 0 = none, 1 = medium, 2 = hotlap, 3 = overtake
    ers_harvested_this_lap_mguk: f32,
    ers_harvested_this_lap_mguh: f32,
    ers_deployed_this_lap: f32,
    network_paused: u8, // Whether the car is paused in a network game
}

// Size: 1239 bytes
#[derive(Debug, Serialize, Deserialize)]
pub struct PacketCarStatusData {
    pub header: PacketHeader,
    car_status_data: Vec<CarStatusData>,
}

impl PacketCarStatusData {
    pub fn from_bytes(data: &[u8]) -> Self {
        let header = PacketHeader::from_bytes(&data[0..29]);
        let mut car_status_data = Vec::new();
        let mut offset = 29;

        for _i in 0..21 {
            let car_status = CarStatusData::from_bytes(&data[offset..offset + 55]);
            car_status_data.push(car_status);
            offset += 53;
        }

        PacketCarStatusData {
            header,
            car_status_data,
        }
    }
}

impl CarStatusData {
    fn from_bytes(data: &[u8]) -> Self {
        CarStatusData {
            traction_control: data[0],
            anti_lock_brakes: data[1],
            fuel_mix: data[2],
            front_brake_bias: data[3],
            pit_limiter_status: data[4],
            fuel_in_tank: LittleEndian::read_f32(&data[5..9]),
            fuel_capacity: LittleEndian::read_f32(&data[9..13]),
            fuel_remaining_laps: LittleEndian::read_f32(&data[13..17]),
            max_rpm: LittleEndian::read_u16(&data[17..19]),
            idle_rpm: LittleEndian::read_u16(&data[19..21]),
            max_gears: data[21],
            drs_allowed: data[22],
            drs_activation_distance: LittleEndian::read_u16(&data[23..25]),
            actual_tyre_compund: data[25],
            visual_tyre_compund: data[26],
            tyres_age_laps: data[27],
            vehicle_fia_flags: data[28] as i8,
            engine_power_ice: LittleEndian::read_f32(&data[29..33]),
            engine_power_mguk: LittleEndian::read_f32(&data[33..37]),
            ers_store_energy: LittleEndian::read_f32(&data[37..41]),
            ers_deploy_mode: data[41],
            ers_harvested_this_lap_mguk: LittleEndian::read_f32(&data[42..46]),
            ers_harvested_this_lap_mguh: LittleEndian::read_f32(&data[46..50]),
            ers_deployed_this_lap: LittleEndian::read_f32(&data[50..54]),
            network_paused: data[54],
        }
    }
}
