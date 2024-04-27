// Frequency: Rate as specified in menu
// Size: 217 bytes
#[derive(Debug)]
pub struct PacketMotionExData {
    header: PacketHeader,
    suspension_velocity: vec<f32>, // <RL, RR, FL, FR> [4]
    suspension_acceleration: vec<f32>,
    wheel_speed: vec<f32>,
    wheel_slip_angle: vec<f32>,
    wheel_lat_force: vec<f32>,
    wheel_long_force: vec<f32>,
    height_of_cog: f32, // Height of COG (Centre of Gravity) from the ground
    local_velocity_x: f32,
    local_velocity_y: f32,
    local_velocity_z: f32,
    angular_velocity_x: f32,
    angular_velocity_y: f32,
    angular_velocity_z: f32,
    angular_acceleration_x: f32,
    angular_acceleration_y: f32,
    angular_acceleration_z: f32,
    front_wheels_angle: f32, // Angle of front wheels in radians
    wheel_vert_force: vec<f32>,
}
