use crate::packets::header::PacketHeader;
use byteorder::{ByteOrder, LittleEndian};
use serde::{Deserialize, Serialize};

// Frequency: 10/s
// Size: 42 bytes
#[derive(Debug, Serialize, Deserialize)]
pub struct CarDamageData {
    tyres_wear: Vec<f32>, // Vec of four floats that represent tyre wear (Percentage)
    tyres_damage: Vec<u8>,
    brakes_damage: Vec<u8>,
    fl_wing_damage: u8, // (Percentage)
    fr_wing_damage: u8,
    rear_wing_damage: u8,
    floor_damage: u8,
    diffuser_damage: u8,
    sidepod_damage: u8,
    drs_fault: u8, // 0 = OK, 1 = fault
    ers_fault: u8,
    gear_box_damage: u8, // (Percentage)
    engine_damage: u8,
    engine_mguh_wear: u8,
    engine_es_wear: u8,
    engine_ce_wear: u8,
    engine_ice_wear: u8,
    engine_mguk_wear: u8,
    engine_tc_wear: u8,
    engine_blown: u8,  // 0 = OK, 1 = Fault
    engine_seized: u8, // 0 = OK, 1 = fault
}

// Size: 953 byte
#[derive(Debug, Serialize, Deserialize)]
pub struct PacketCarDamageData {
    pub header: PacketHeader,
    car_damage_data: Vec<CarDamageData>,
}

impl PacketCarDamageData {
    pub fn from_bytes(data: &[u8]) -> Self {
        let header = PacketHeader::from_bytes(&data[0..29]);
        let mut car_damage_data = Vec::new();
        let mut offset = 29;

        for _i in 0..21 {
            let data = CarDamageData::from_bytes(&data[offset..offset + 42]);
            car_damage_data.push(data);
            offset += 42;
        }

        PacketCarDamageData {
            header: header,
            car_damage_data: car_damage_data,
        }
    }
}

impl CarDamageData {
    fn from_bytes(data: &[u8]) -> Self {
        let mut tyres_wear = Vec::new();
        let mut tyres_damage = Vec::new();
        let mut brakes_damage = Vec::new();

        for i in 0..3 {
            tyres_wear.push(LittleEndian::read_f32(&data[i..i + 4]));
            tyres_damage.push(data[i + 16]);
            brakes_damage.push(data[i + 20]);
        }

        CarDamageData {
            tyres_wear: tyres_wear,
            tyres_damage: tyres_damage,
            brakes_damage: brakes_damage,
            fl_wing_damage: data[24],
            fr_wing_damage: data[25],
            rear_wing_damage: data[26],
            floor_damage: data[27],
            diffuser_damage: data[28],
            sidepod_damage: data[29],
            drs_fault: data[30],
            ers_fault: data[31],
            gear_box_damage: data[32],
            engine_damage: data[33],
            engine_mguh_wear: data[34],
            engine_es_wear: data[35],
            engine_ce_wear: data[36],
            engine_ice_wear: data[37],
            engine_mguk_wear: data[38],
            engine_tc_wear: data[39],
            engine_blown: data[40],
            engine_seized: data[41],
        }
    }
}
