use serde::Serialize;

use crate::packets::header::PacketHeader;

// Frequency: Rate as specified in menu
// Size: 217 bytes
#[derive(Debug, Serialize)]
pub struct PacketMotionExData {
    header: PacketHeader,
    suspension_velocity: Vec<f32>, // <RL, RR, FL, FR> [4]
    suspension_acceleration: Vec<f32>,
    wheel_speed: Vec<f32>,
    wheel_slip_angle: Vec<f32>,
    wheel_lat_force: Vec<f32>,
    wheel_long_force: Vec<f32>,
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
    wheel_vert_force: Vec<f32>,
}
