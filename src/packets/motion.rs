// For the normalised vectors, to convert to float values
// divide by 32767.0f - 16 bit signed values are used to pack the
// data and on the assumption that direction values are always
// between -1.0f and 1.0f.
// Size: 1349 bytes
#[derive(Debug)]
pub struct CarMotionData {
    // World space x pos (m)
    world_position_x: f32,
    // World space y pos (m)
    world_position_y: f32,
    // World space z pos (m)
    world_position_z: f32,
    // x velocity (m/s)
    world_velocity_x: f32,
    // y velocity (m/s)
    world_velocity_y: f32,
    // z velocity (m/s)
    world_velocity_z: f32,
    // World space forward x direction (normalised)
    world_forward_dir_x: i16,
    // World space forward y direction (normalised)
    world_forward_dir_y: i16,
    // World space forward z direction (normalised)
    world_forward_dir_z: i16,
    // World space right x direction (normalised)
    world_right_dir_x: i16,
    // World space right y direction (normalised)
    world_right_dir_x: i16,
    // World space right z direction (normalised)
    world_right_dir_x: i16,
    // G forces
    lateral_g_force: f32,
    longitudinal_g_force: f32,
    vertical_g_force: f32,
    // Yaw angle in radians
    yaw: f32,
    // Pitch angle in radians
    pitch: f32,
    // Roll angle in radians
    roll: f32,
}

#[derive(Debug)]
pub struct PacketMotionData {
    header: PacketHeader,
    car_motion_data: Vec<CarMotionData>,
}
