use std::{
    fs::{self, OpenOptions},
    io::{Seek, SeekFrom},
    path::Path,
};

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

#[derive(Deserialize)]
struct Config {
    motion_data_path: String,
    session_data_path: String,
    lap_data_path: String,
    event_data_path: String,
    participants_data_path: String,
    setups_data_path: String,
    telemetry_data_path: String,
    car_status_data_path: String,
    final_classification_data_path: String,
    lobby_info_data_path: String,
    car_damage_data_path: String,
    session_history_data_path: String,
    tyre_sets_data_path: String,
    motion_ex_data_path: String,
}

impl SerializeToJSON for Packet {
    fn serialize_to_json(&self) -> std::io::Result<()> {
        let config_str =
            fs::read_to_string("./src/config.json").expect("Failed to read config file");
        let config: Config =
            serde_json::from_str(&config_str).expect("Failed to parse config file");

        let (file_path, session_uid) = match self {
            Packet::Motion(data) => (&config.motion_data_path, data.header.session_uid),
            Packet::Session(data) => (&config.session_data_path, data.header.session_uid),
            Packet::LapData(data) => (&config.lap_data_path, data.header.session_uid),
            Packet::Event(data) => (&config.event_data_path, data.header.session_uid),
            Packet::Participants(data) => (&config.participants_data_path, data.header.session_uid),
            Packet::Setups(data) => (&config.setups_data_path, data.header.session_uid),
            Packet::Telemetry(data) => (&config.telemetry_data_path, data.header.session_uid),
            Packet::Status(data) => (&config.car_status_data_path, data.header.session_uid),
            Packet::FinalClassification(data) => (
                &config.final_classification_data_path,
                data.header.session_uid,
            ),
            Packet::Lobby(data) => (&config.lobby_info_data_path, data.header.session_uid),
            Packet::Damage(data) => (&config.car_damage_data_path, data.header.session_uid),
            Packet::History(data) => (&config.session_history_data_path, data.header.session_uid),
            Packet::TyreSets(data) => (&config.tyre_sets_data_path, data.header.session_uid),
            Packet::MotionEx(data) => (&config.motion_ex_data_path, data.header.session_uid),
        };

        let file_name = format!("{}{}.json", file_path, session_uid);
        let path = Path::new(&file_name);
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

pub trait SerializeToJSON {
    fn serialize_to_json(&self) -> std::io::Result<()>;
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
