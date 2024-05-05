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
use serde::Serialize;

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

pub trait SerializeToJSON {
    fn serialize_to_json(&self) -> std::io::Result<()>;
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
