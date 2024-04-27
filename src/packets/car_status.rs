// Frequency: Rate as specified in menu
// Size: 1239 bytes
#[derive(Debug)]
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
    actual_tyre_compund: TyreCompound,
    visual_tyre_compund: TyreCompound,
    tyres_age_laps: u8,
    vehicle_fia_flags: u8, // -1 = invalid, 0 = none, 1 = green, 2 = blue, 3 = yellow
    engine_power_ice: f32, // (W)
    engine_power_mguk: f32, // (W)
    ers_store_energy: f32, // ERS energy store in Joules
    ers_deploy_mode: u8,   // 0 = none, 1 = medium, 2 = hotlap, 3 = overtake
    ers_harvested_this_lap_mguk: f32,
    ers_harvested_this_lap_mguh: f32,
    ers_deployed_this_lap: f32,
    network_paused: u8, // Whether the car is paused in a network game
}

#[derive(Debug)]
pub struct PacketCarStatusData {
    header: PacketHeader,
    car_status_data: CarStatusData,
}
