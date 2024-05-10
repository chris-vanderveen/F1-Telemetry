use crate::packets::header::PacketHeader;
use byteorder::{ByteOrder, LittleEndian};
use serde::{Deserialize, Serialize};

// Frequency: Rate as specified in menu
// Size: 217 bytes
#[derive(Debug, Serialize, Deserialize)]
pub struct PacketMotionExData {
    pub header: PacketHeader,
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

impl PacketMotionExData {
    pub fn from_bytes(data: &[u8]) -> Self {
        let header = PacketHeader::from_bytes(&data[0..29]);
        let mut suspension_velocity = Vec::new();
        let mut suspension_acceleration = Vec::new();
        let mut wheel_speed = Vec::new();
        let mut wheel_slip_angle = Vec::new();
        let mut wheel_lat_force = Vec::new();
        let mut wheel_long_force = Vec::new();
        let mut wheel_vert_force = Vec::new();

        for i in 0..3 {
            let sus_vel = LittleEndian::read_f32(&data[i..i + 4]);
            suspension_velocity.push(sus_vel);
            let sus_acc = LittleEndian::read_f32(&data[i + 16..i + 20]);
            suspension_acceleration.push(sus_acc);
            let w_speed = LittleEndian::read_f32(&data[i + 32..i + 36]);
            wheel_speed.push(w_speed);
            let w_slip_angle = LittleEndian::read_f32(&data[i + 48..i + 52]);
            wheel_slip_angle.push(w_slip_angle);
            let w_lat_force = LittleEndian::read_f32(&data[i + 64..i + 68]);
            wheel_lat_force.push(w_lat_force);
            let w_long_force = LittleEndian::read_f32(&data[i + 80..i + 84]);
            wheel_long_force.push(w_long_force);
            let w_vert_force = LittleEndian::read_f32(&data[i + 128..i + 132]);
            wheel_vert_force.push(w_vert_force);
        }

        PacketMotionExData {
            header: header,
            suspension_velocity: suspension_velocity,
            suspension_acceleration: suspension_acceleration,
            wheel_speed: wheel_speed,
            wheel_slip_angle: wheel_slip_angle,
            wheel_lat_force: wheel_lat_force,
            wheel_long_force: wheel_long_force,
            height_of_cog: LittleEndian::read_f32(&data[84..88]),
            local_velocity_x: LittleEndian::read_f32(&data[88..92]),
            local_velocity_y: LittleEndian::read_f32(&data[92..96]),
            local_velocity_z: LittleEndian::read_f32(&data[96..100]),
            angular_velocity_x: LittleEndian::read_f32(&data[100..104]),
            angular_velocity_y: LittleEndian::read_f32(&data[104..108]),
            angular_velocity_z: LittleEndian::read_f32(&data[108..112]),
            angular_acceleration_x: LittleEndian::read_f32(&data[112..116]),
            angular_acceleration_y: LittleEndian::read_f32(&data[116..120]),
            angular_acceleration_z: LittleEndian::read_f32(&data[120..124]),
            front_wheels_angle: LittleEndian::read_f32(&data[124..128]),
            wheel_vert_force: wheel_vert_force,
        }
    }
}
