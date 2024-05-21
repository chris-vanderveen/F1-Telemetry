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
    fs::OpenOptions,
    io::{Seek, SeekFrom},
    path::Path,
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
            Packet::Motion(data) => (env::var("MOTION_PATH"), data.header.session_uid),
            Packet::Session(data) => (env::var("SESSION_PATH"), data.header.session_uid),
            Packet::LapData(data) => (env::var("LAP_DATA_PATH"), data.header.session_uid),
            Packet::Event(data) => (env::var("EVENT_PATH"), data.header.session_uid),
            Packet::Participants(data) => (env::var("PARTICIPANTS_PATH"), data.header.session_uid),
            Packet::Setups(data) => (env::var("CAR_SETUPS_PATH"), data.header.session_uid),
            Packet::Telemetry(data) => (env::var("CAR_TELEMETRY_PATH"), data.header.session_uid),
            Packet::Status(data) => (env::var("CAR_STATUS_PATH"), data.header.session_uid),
            Packet::FinalClassification(data) => (
                env::var("FINAL_CLASSIFICATION_PATH"),
                data.header.session_uid,
            ),
            Packet::Lobby(data) => (env::var("LOBBY_INFO_PATH"), data.header.session_uid),
            Packet::Damage(data) => (env::var("CAR_DAMAGE_PATH"), data.header.session_uid),
            Packet::History(data) => (env::var("SESSION_HISTORY_PATH"), data.header.session_uid),
            Packet::TyreSets(data) => (env::var("TYRE_SETS_PATH"), data.header.session_uid),
            Packet::MotionEx(data) => (env::var("MOTION_EX_PATH"), data.header.session_uid),
        };

        let file_name = format!(
            "{}{:?}{}.json",
            env::var("STORAGE_ROOT").expect("STORAGE_ROOT environment variable not set"),
            file_path,
            session_uid
        );
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
