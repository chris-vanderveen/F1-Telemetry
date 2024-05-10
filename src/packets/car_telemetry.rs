use crate::packets::header::PacketHeader;
use byteorder::ByteOrder;
use byteorder::LittleEndian;
use serde::Deserialize;
use serde::Serialize;

// Frequency: 2/s
// Size: 60 bytes
#[derive(Debug, Serialize, Deserialize)]
pub struct CarTelemetryData {
    speed: u16,    // Speed of car in km/h
    throttle: f32, // Amount of throttle applied (0.0 -> 1.0)
    steer: f32,    // -1.0 = full lock left, 1.0 = full lock right
    brake: f32,    // (0.0 -> 1.0)
    clutch: u8,    // 0 -> 100
    gear: i8,      // (1-8, N=0, R=-1)
    engine_rpm: u16,
    drs: u8, // 0 = off, 1 = on
    rev_lights_percent: u8,
    rev_lights_bit_value: u16, // bit 0 = leftmost LED, bit 14 = rightmost LED
    brakes_temperature: WheelData<u16>, // Brakes temp (Celsius)
    tyres_surface_temp: WheelData<u8>, // Surface temp (Celsius)
    tyres_inner_temp: WheelData<u8>,
    engine_temp: u16,
    tyres_pressure: WheelData<f32>,
    surface_type: WheelData<u8>,
}

// Size: 1352 bytes
#[derive(Debug, Serialize, Deserialize)]
pub struct PacketCarTelemetryData {
    pub header: PacketHeader,
    car_telemetry_data: Vec<CarTelemetryData>,
    mfd_panel_index: u8, // Index of MFD panel (255 = Closed, 0 = car setup, 1 = pits, 2 = damage, 3 = engine, 4 = Temperatures)
    mfd_panel_secondary_user: u8,
    suggested_gear: i8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WheelData<T> {
    rear_left: T,
    rear_right: T,
    front_left: T,
    front_right: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SurfaceType {
    Tarmac,      // 0
    RumbleStrip, // 1
    Concrete,    // 2
    Rock,        // 3
    Gravel,      // 4
    Mud,         // 5
    Sand,        // 6
    Grass,       // 7
    Water,       // 8
    Cobblestone, // 9
    Metal,       // 10
    Ridged,      // 11
}

impl PacketCarTelemetryData {
    pub fn from_bytes(data: &[u8]) -> Self {
        let header = PacketHeader::from_bytes(&data[0..29]);
        let mut car_telemetry_data = Vec::new();
        let mut offset = 29;

        for _i in 0..22 {
            let car_data = CarTelemetryData::from_bytes(&data[offset..offset + 60]);
            car_telemetry_data.push(car_data);
            offset += 60;
        }

        PacketCarTelemetryData {
            header: header,
            car_telemetry_data: car_telemetry_data,
            mfd_panel_index: data[1349],
            mfd_panel_secondary_user: data[1350],
            suggested_gear: data[1351] as i8,
        }
    }
}

impl CarTelemetryData {
    fn from_bytes(data: &[u8]) -> Self {
        let brakes_temperature = WheelData {
            rear_left: LittleEndian::read_u16(&data[22..24]),
            rear_right: LittleEndian::read_u16(&data[24..26]),
            front_left: LittleEndian::read_u16(&data[26..28]),
            front_right: LittleEndian::read_u16(&data[28..30]),
        };

        let tyres_surface_temp = WheelData {
            rear_left: data[30],
            rear_right: data[31],
            front_left: data[32],
            front_right: data[33],
        };

        let tyres_inner_temp = WheelData {
            rear_left: data[34],
            rear_right: data[35],
            front_left: data[36],
            front_right: data[37],
        };

        let tyres_pressure = WheelData {
            rear_left: LittleEndian::read_f32(&data[40..44]),
            rear_right: LittleEndian::read_f32(&data[44..48]),
            front_left: LittleEndian::read_f32(&data[48..52]),
            front_right: LittleEndian::read_f32(&data[52..56]),
        };

        let surface_type = WheelData {
            rear_left: data[56],
            rear_right: data[57],
            front_left: data[58],
            front_right: data[59],
        };

        CarTelemetryData {
            speed: LittleEndian::read_u16(&data[0..2]),
            throttle: LittleEndian::read_f32(&data[2..6]),
            steer: LittleEndian::read_f32(&data[6..10]),
            brake: LittleEndian::read_f32(&data[10..14]),
            clutch: data[14],
            gear: data[15] as i8,
            engine_rpm: LittleEndian::read_u16(&data[16..18]),
            drs: data[18],
            rev_lights_percent: data[19],
            rev_lights_bit_value: LittleEndian::read_u16(&data[20..22]),
            brakes_temperature: brakes_temperature,
            tyres_surface_temp: tyres_surface_temp,
            tyres_inner_temp: tyres_inner_temp,
            engine_temp: LittleEndian::read_u16(&data[38..40]),
            tyres_pressure: tyres_pressure,
            surface_type: surface_type,
        }
    }
}
