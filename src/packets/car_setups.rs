use crate::packets::header::PacketHeader;

// Frequency: 2/s

// Size: 1107 bytes
#[derive(Debug)]
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

#[derive(Debug)]
pub struct PacketCarSetupData {
    header: PacketHeader,
    car_setups: CarSetupData,
}
