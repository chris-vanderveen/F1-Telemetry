use crate::packets::car_damage_data::PacketCarDamageData;
use crate::packets::car_setups::PacketCarSetupData;
use crate::packets::car_status::PacketCarStatusData;
use crate::packets::car_telemetry::PacketCarTelemetryData;
use crate::packets::event::PacketEventData;
use crate::packets::final_classication::PacketFinalClassification;
use crate::packets::lap_data::PacketLapData;
use crate::packets::lobby_info::PacketLobbyInfoData;
use crate::packets::motion::PacketMotionData;
use crate::packets::motion_ex::PacketMotionExData;
use crate::packets::participants::PacketParticipantsData;
use crate::packets::session::PacketSessionData;
use crate::packets::session_history::PacketSessionHistoryData;
use crate::packets::tyre_sets::PacketTyreSetData;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::{
    env,
    fs::{File, OpenOptions},
    io::{Read, Seek, SeekFrom},
    path::PathBuf,
};

#[derive(Debug, Serialize, Deserialize)]
pub enum Packet {
    Motion(PacketMotionData),
    Session(PacketSessionData),
    LapData(PacketLapData),
    Event(PacketEventData),
    Participants(PacketParticipantsData),
    Setups(PacketCarSetupData),
    Telemetry(PacketCarTelemetryData),
    Status(PacketCarStatusData),
    FinalClassification(PacketFinalClassification),
    Lobby(PacketLobbyInfoData),
    Damage(PacketCarDamageData),
    History(PacketSessionHistoryData),
    TyreSets(PacketTyreSetData),
    MotionEx(PacketMotionExData),
}

impl SerializeToJSON for Packet {
    fn serialize_to_json(&self) -> std::io::Result<()> {
        let (file_path, session_uid) = match self {
            Packet::Motion(data) => (
                env::var("MOTION_PATH").expect("MOTION_PATH not set"),
                data.header.session_uid,
            ),
            Packet::Session(data) => (
                env::var("SESSION_PATH").expect("SESSION_PATH not set"),
                data.header.session_uid,
            ),
            Packet::LapData(data) => (
                env::var("LAP_DATA_PATH").expect("LAP_DATA_PATH not set"),
                data.header.session_uid,
            ),
            Packet::Event(data) => (
                env::var("EVENT_PATH").expect("EVENT_PATH not set"),
                data.header.session_uid,
            ),
            Packet::Participants(data) => (
                env::var("PARTICIPANTS_PATH").expect("PARTICIPANTS_PATH not set"),
                data.header.session_uid,
            ),
            Packet::Setups(data) => (
                env::var("CAR_SETUPS_PATH").expect("CAR_SETUPS_PATH not set"),
                data.header.session_uid,
            ),
            Packet::Telemetry(data) => (
                env::var("CAR_TELEMETRY_PATH").expect("CAR_TELEMETRY_PATH not set"),
                data.header.session_uid,
            ),
            Packet::Status(data) => (
                env::var("CAR_STATUS_PATH").expect("CAR_STATUS_PATH not set"),
                data.header.session_uid,
            ),
            Packet::FinalClassification(data) => (
                env::var("FINAL_CLASSIFICATION_PATH").expect("FINAL_CLASSIFICATION_PATH not set"),
                data.header.session_uid,
            ),
            Packet::Lobby(data) => (
                env::var("LOBBY_INFO_PATH").expect("LOBBY_INFO_PATH not set"),
                data.header.session_uid,
            ),
            Packet::Damage(data) => (
                env::var("CAR_DAMAGE_PATH").expect("CAR_DAMAGE_PATH not set"),
                data.header.session_uid,
            ),
            Packet::History(data) => (
                env::var("SESSION_HISTORY_PATH").expect("SESSION_HISTORY_PATH not set"),
                data.header.session_uid,
            ),
            Packet::TyreSets(data) => (
                env::var("TYRE_SETS_PATH").expect("TYRE_SETS_PATH not set"),
                data.header.session_uid,
            ),
            Packet::MotionEx(data) => (
                env::var("MOTION_EX_PATH").expect("MOTION_EX_PATH not set"),
                data.header.session_uid,
            ),
        };

        let root_path =
            env::var("STORAGE_ROOT").expect("STORAGE_ROOT environment variable not set");
        let file_name = format!("{:?}.json", session_uid);
        let mut path = PathBuf::from(root_path);
        path.push(file_path);
        path.push(file_name);

        let file_exists = path.exists();

        if file_exists {
            // Open the file and read its content
            let mut file = OpenOptions::new().read(true).write(true).open(&path)?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;

            // Check if the file ends with a closing bracket
            if let Some(last_char) = content.chars().last() {
                if last_char == ']' {
                    // Remove the closing bracket
                    content.pop();
                    // If there are other elements in the array, add a comma
                    if content.len() > 1 {
                        content.push(',');
                    }
                }
            }

            // Append the new JSON object and close the array
            let new_object = serde_json::to_string(self)?;
            content.push_str(&new_object);
            content.push(']');

            // Rewrite the file with the new content
            file.seek(SeekFrom::Start(0))?;
            file.write_all(content.as_bytes())?;
        } else {
            // Create a new file and write the JSON object in array format
            let mut file = File::create(&path)?;
            let new_object = serde_json::to_string(self)?;
            let content = format!("[{}]", new_object);
            file.write_all(content.as_bytes())?;
        }

        Ok(())
    }
}

pub trait SerializeToJSON {
    fn serialize_to_json(&self) -> std::io::Result<()>;
}

pub trait PacketProcessor {
    fn process_packet(packet_data: &[u8]);
}

// Just some extra (necessary) stuff that I don't know what else to do with
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
