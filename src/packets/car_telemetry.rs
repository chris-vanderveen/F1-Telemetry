use serde::Serialize;

use crate::packet::SurfaceType;
use crate::packet::WheelData;
use crate::packets::header::PacketHeader;

// Frequency: 2/s
// Size: 1107 bytes
#[derive(Debug, Serialize)]
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
    surface_type: WheelData<SurfaceType>,
}

#[derive(Debug, Serialize)]
pub struct PacketCarTelemetryData {
    header: PacketHeader,
    car_telemetry_data: CarTelemetryData,
    mfd_panel_index: u8, // Index of MFD panel (255 = Closed, 0 = car setup, 1 = pits, 2 = damage, 3 = engine, 4 = Temperatures)
    mfd_panel_secondary_user: u8,
    suggested_gear: i8,
}
