use crate::{packet::SerializeToJSON, packets::header::PacketHeader};
use byteorder::{ByteOrder, LittleEndian};
use serde::Serialize;
use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom, Write};
use std::path::Path;

// For the normalised vectors, to convert to float values
// divide by 32767.0f - 16 bit signed values are used to pack the
// data and on the assumption that direction values are always
// between -1.0f and 1.0f.
// Size: 1349 bytes

// 60 bytes each * 22 = 1320
#[derive(Debug, Serialize)]
pub struct CarMotionData {
    // World space x pos (m)
    pub world_position_x: f32,
    // World space y pos (m)
    pub world_position_y: f32,
    // World space z pos (m)
    pub world_position_z: f32,
    // x velocity (m/s)
    pub world_velocity_x: f32,
    // y velocity (m/s)
    pub world_velocity_y: f32,
    // z velocity (m/s)
    pub world_velocity_z: f32,
    // World space forward x direction (normalised)
    pub world_forward_dir_x: i16,
    // World space forward y direction (normalised)
    pub world_forward_dir_y: i16,
    // World space forward z direction (normalised)
    pub world_forward_dir_z: i16,
    // World space right x direction (normalised)
    pub world_right_dir_x: i16,
    // World space right y direction (normalised)
    pub world_right_dir_y: i16,
    // World space right z direction (normalised)
    pub world_right_dir_z: i16,
    // G forces
    pub lateral_g_force: f32,
    pub longitudinal_g_force: f32,
    pub vertical_g_force: f32,
    // Yaw angle in radians
    pub yaw: f32,
    // Pitch angle in radians
    pub pitch: f32,
    // Roll angle in radians
    pub roll: f32,
}

#[derive(Debug, Serialize)]
pub struct PacketMotionData {
    header: PacketHeader,
    car_motion_data: Vec<CarMotionData>,
}

impl CarMotionData {
    pub fn from_bytes(data: &[u8]) -> Self {
        CarMotionData {
            world_position_x: LittleEndian::read_f32(&data[0..4]),
            world_position_y: LittleEndian::read_f32(&data[4..8]),
            world_position_z: LittleEndian::read_f32(&data[8..12]),
            world_velocity_x: LittleEndian::read_f32(&data[12..16]),
            world_velocity_y: LittleEndian::read_f32(&data[16..20]),
            world_velocity_z: LittleEndian::read_f32(&data[20..24]),
            world_forward_dir_x: LittleEndian::read_i16(&data[24..26]),
            world_forward_dir_y: LittleEndian::read_i16(&data[26..28]),
            world_forward_dir_z: LittleEndian::read_i16(&data[28..30]),
            world_right_dir_x: LittleEndian::read_i16(&data[30..32]),
            world_right_dir_y: LittleEndian::read_i16(&data[32..34]),
            world_right_dir_z: LittleEndian::read_i16(&data[34..36]),
            lateral_g_force: LittleEndian::read_f32(&data[36..40]),
            longitudinal_g_force: LittleEndian::read_f32(&data[40..44]),
            vertical_g_force: LittleEndian::read_f32(&data[44..48]),
            yaw: LittleEndian::read_f32(&data[48..52]),
            pitch: LittleEndian::read_f32(&data[52..56]),
            roll: LittleEndian::read_f32(&data[56..60]),
        }
    }
}

impl PacketMotionData {
    pub fn from_bytes(data: &[u8]) -> Self {
        let header = PacketHeader::from_bytes(&data[0..29]);
        let mut car_motion_data = Vec::new();
        let mut offset = 29;
        for _ in 0..22 {
            let motion_data = CarMotionData::from_bytes(&data[offset..offset + 60]);
            car_motion_data.push(motion_data);
            offset += 60;
        }

        PacketMotionData {
            header: header,
            car_motion_data: car_motion_data,
        }
    }
}

impl SerializeToJSON for PacketMotionData {
    fn serialize_to_json(&self) -> std::io::Result<()> {
        let path_name = format!(
            "/Users/chrisvanderveen/Documents/School/DEV/f1_data/motion_data/{}_motion.json",
            self.header.session_uid
        );
        let path = Path::new(&path_name);
        let file_exists = path.exists();
        let mut file = OpenOptions::new()
            .write(true)
            .create(!file_exists)
            .append(file_exists)
            .open(path)?;

        if file_exists {
            file.seek(SeekFrom::End(-1))?;
            writeln!(file, ",{}", serde_json::to_string(self)?)?;
        } else {
            writeln!(file, "[{},", serde_json::to_string(self)?)?;
        }

        Ok(())
    }
}
