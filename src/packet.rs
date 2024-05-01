use crate::packets::header::PacketHeader;
use crate::packets::motion::CarMotionData;
use serde::Serialize;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::Path;

pub struct Packet<'a> {
    header: PacketHeader,
    data: &'a [u8],
}

impl<'a> Packet<'a> {
    fn new(header: PacketHeader, data: &'a [u8]) -> Self {
        Packet { header, data }
    }

    fn process(&self) {
        match self.header.packet_id {
            0 => self.process_motion_data(),
            // 1 => self.process_session_data(),
            // 2 => self.process_lap_data(),
            // 3 => self.process_event_data(),
            // 4 => self.process_participants_data(),
            // 5 => self.process_car_setups_data(),
            // 6 => self.process_car_telemetry_data(),
            // 7 => self.process_car_status_data(),
            // 8 => self.process_final_classification_data(),
            // 9 => self.process_lobby_info_data(),
            // 10 => self.process_car_damage_data(),
            // 11 => self.process_session_history_data(),
            // 12 => self.process_tyre_sets_data(),
            // 13 => self.process_motion_ex_data(),
            _ => println!("Unknown packet ID: {}", self.header.packet_id),
        }
    }

    fn process_motion_data(&self) {
        let car_motion_data = CarMotionData::from_bytes(self.data);
        let filename = format!(
            "/Users/chrisvanderveen/Documents/School/DEV/f1_data/{}_motion.json",
            self.header.session_uid
        );
        let path = Path::new(&filename);
        if let Err(e) = Packet::<'a>::save_to_file(&car_motion_data, &path) {
            eprintln!("Failed to save or update motion data: {}", e);
        }
    }

    fn save_to_file<T: Serialize>(data: &T, path: &Path) -> io::Result<()> {
        let file_exists = path.exists();
        let mut file = OpenOptions::new()
            .write(true)
            .create(!file_exists)
            .append(file_exists)
            .open(path)?;

        if file_exists {
            writeln!(file, ",{}", serde_json::to_string(data)?);
        } else {
            writeln!(file, "[{}]", serde_json::to_string(data)?);
        }

        Ok(())
    }
}

// Just some extra (necessary) stuff that I don't know what else to do with
#[derive(Debug, Serialize)]
pub struct WheelData<T> {
    pub rear_left: T,
    pub rear_right: T,
    pub front_left: T,
    pub front_right: T,
}

#[derive(Debug, Serialize)]
pub enum SurfaceType {
    Tarmac,
    RumbleStrip,
    Concrete,
    Rock,
    Gravel,
    Mud,
    Sand,
    Grass,
    Water,
    Cobblestone,
    Metal,
    Ridged,
}

#[derive(Debug, Default, Serialize)]
pub enum TyreCompound {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    Inter,
    Wet,
    ClassicDry,
    ClassicWet,
    F2SuperSoft,
    F2Soft,
    F2Medium,
    F2Hard,
    F2Wet,
    #[default]
    Invalid,
}
