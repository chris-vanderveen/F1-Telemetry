// Frequency: 10/s
// Size: 953 bytes
#[derive(Debug)]
pub struct CarDamageData {
    tyres_wear: vec<f32>, // Vec of four floats that represent tyre wear (Percentage)
    tyres_damage: vec<u8>,
    brakes_damage: vec<u8>,
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

#[derive(Debug)]
pub struct PackageCarDamageData {
    header: PacketHeader,
    car_damage_data: CarDamageData,
}
