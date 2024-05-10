use byteorder::{ByteOrder, LittleEndian};
use serde::{Deserialize, Serialize};

use crate::packets::header::PacketHeader;

// The Event details packet is different for each type of event
// Make sure the correct type is interpreted.
#[derive(Debug, Serialize, Deserialize)]
pub struct FastestLap {
    vehicle_idx: u8, // Index of car achieving fastest lap
    lap_time: f32,   // Lap time in seconds
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Retirement {
    vehicle_idx: u8, // Index of vehicle retiring
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeamMateInPits {
    vehicle_idx: u8, // Vehicle index of teammate
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RaceWinner {
    vehicle_idx: u8, // Race winner
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Penalty {
    penalty_type: u8,
    infringement_type: u8,
    vehicle_idx: u8,       // Index of the car pen is applied to
    other_vehicle_idx: u8, // Index of other vehicle involved
    time: u8,              // Time gained or spent doing action
    lap_num: u8,           // Lap penalty occurred on
    places_gained: u8,     // Num places gained doing this
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpeedTrap {
    vehicle_idx: u8,                    // Index of vehicle triggering speed trap
    speed: f32,                         // Top speed achieved (km/h)
    is_overall_fastest_in_session: u8,  // true = 1, false = 0
    is_driver_fastest_in_session: u8,   // true = 1, false = 0
    fastest_vehicle_idx_in_session: u8, // Vehicle index of fastest vehicle in session
    fastest_speed_in_session: f32,      // Highest speed of fastest vehicle in session
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartLights {
    num_lights: u8, // Number of lights showing
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DriveThroughPenaltyServed {
    vehicle_idx: u8, // Index of vehicle serving drive through
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StopGoPenaltyServed {
    vehicle_idx: u8, // Index of vehicle serving stop and go
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Flashback {
    flashback_frame_id: u32,     // Frame flashed back to
    flashback_session_time: f32, // Session time flashed to
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Buttons {
    button_status: u32, // Bit flags specifying which buttons are being pressed
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Overtake {
    overtaking_vehicle_idx: u8,      // Index of overtaking vehicle
    being_overtaken_vehicle_idx: u8, // Index of vehicle being overtaken
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Event {
    SessionStarted,
    SessionEnded,
    FastestLap(FastestLap),
    Retirement(Retirement),
    DRSEnabled,
    DRSDisabled,
    TeamMateInPits(TeamMateInPits),
    ChequeredFlag,
    RaceWinner(RaceWinner),
    Penalty(Penalty),
    SpeedTrap(SpeedTrap),
    StartLights(StartLights),
    LightsOut,
    DriveThroughPenaltyServed(DriveThroughPenaltyServed),
    StopGoPenaltyServed(StopGoPenaltyServed),
    Flashback(Flashback),
    Buttons(Buttons),
    RedFlag,
    Overtake(Overtake),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PacketEventData {
    pub header: PacketHeader,
    event_string_code: String,
    event: Event,
}

// Not sure if I actually need this or not
#[derive(Debug, Deserialize, Serialize)]
pub enum EventStringCode {
    SSTA, // Session Started - Sent when the session starts
    SEND, // Session Ended - Sent when the session ends
    FTLP, // Fastest Lap - When a driver achieves the fastest lap
    RTMT, // Retirement - When a driver retires
    DRSE, // DRS Enabled
    DRSD, // DRS Disabled
    TMPT, // Team Mate in Pits
    CHQF, // Checquered Flag
    RCWN, // Race Winner - Race winner is announced
    PENA, // Penalty issued
    SPTP, // Speed Trap Triggered
    STLG, // Start Lights
    LGOT, // Lights Out
    DTSV, // Drive Through Served
    SGSV, // Stop Go Served
    FLBK, // Flashback
    BUTN, // Button Status
    RDFL, // Red Flag
    OVTK, // Overtake - Overtake Occurred
}

// List of possible penalties
#[derive(Debug, Serialize, Deserialize)]
pub enum PenaltyType {
    DriveThrough,
    StopGo,
    GridPenalty,
    PenaltyReminder,
    TimePenalty,
    Warning,
    Disqualified,
    RemovedFromFormationLap,
    ParkedTooLongTimer,
    TyreRegulations,
    ThisLapInvalidated,
    ThisAndNextLapInvalidated,
    ThisLapInvalidatedWithoutReason,
    ThisAndNextLapInvalidatedWithoutReason,
    ThisAndPreviousLapInvalidated,
    ThisAndPreviousLapInvalidatedWithoutReason,
    Retired,
    BlackFlagTimer,
}

// List of possible infringments
#[derive(Debug, Serialize, Deserialize)]
pub enum InfringementType {
    BlockingBySlowDriving,
    BlockingByWrongWayDriving,
    ReversingOffTheStartLine,
    BigCollision,
    SmallCollision,
    CollisionFailedToHandBackPositionSingle,
    CollisionFailedToHandBackPositionMultiple,
    CornerCuttingGainedTime,
    CornerCuttingOvertakeSingle,
    CornerCuttingOvertakeMultiple,
    CrossedPitExitLane,
    IgnoringBlueFlags,
    IgnoringYellowFlags,
    IgnoringDriveThrough,
    TooManyDriveThroughs,
    DriveThroughReminderServeWithinNLaps,
    DriveThroughReminderServeThisLap,
    PitLaneSpeeding,
    ParkedForTooLong,
    IgnoringTyreRegulations,
    TooManyPenalties,
    MultipleWarnings,
    ApproachingDisqualification,
    TyreRegulationsSelectSingle,
    TyreRegulationsSelectMultiple,
    LapInvalidatedCornerCutting,
    LapInvalidatedRunningWide,
    CornerCuttingRanWideGainedTimeMinor,
    CornerCuttingRanWideGainedTimeSignificant,
    CornerCuttingRanWideGainedTimeExtreme,
    LapInvalidatedWallRiding,
    LapInvalidatedFlashbackUsed,
    LapInvalidatedResetToTrack,
    BlockingThePitlane,
    JumpStart,
    SafetyCarToCarCollision,
    SafetyCarIllegalOvertake,
    SafetyCarExceedingAllowedPace,
    VirtualSafetyCarExceedingAllowedPace,
    FormationLapBelowAllowedSpeed,
    RetiredMechanicalFailure,
    RetiredTerminallyDamaged,
    SafetyCarFallingTooFarBack,
    BlackFlagTimer,
    UnservedStopGoPenalty,
    UnservedDriveThroughPenalty,
    EngineComponentChange,
    GearboxChange,
    LeagueGridPenalty,
    RetryPenalty,
    IllegalTimeGain,
    MandatoryPitstop,
    FormationLapParking,
    ParcFermeChange,
    AttributeAssigned,
}

impl PacketEventData {
    pub fn from_bytes(data: &[u8]) -> Self {
        let header = PacketHeader::from_bytes(&data[0..29]);
        let event_string = std::str::from_utf8(&data[29..33]).unwrap();

        let event = match event_string {
            "SSTA" => Event::SessionStarted,
            "SEND" => Event::SessionEnded,
            "FTLP" => {
                let vehicle_idx = data[33];
                let lap_time = LittleEndian::read_f32(&data[34..38]);
                Event::FastestLap(FastestLap {
                    vehicle_idx,
                    lap_time,
                })
            }
            "RTMT" => {
                let vehicle_idx = data[33];
                Event::Retirement(Retirement { vehicle_idx })
            }
            "DRSE" => Event::DRSEnabled,
            "DRSD" => Event::DRSDisabled,
            "TMTP" => {
                let vehicle_idx = data[33];
                Event::TeamMateInPits(TeamMateInPits { vehicle_idx })
            }
            "CHQF" => Event::ChequeredFlag,
            "RCWN" => {
                let vehicle_idx = data[33];
                Event::RaceWinner(RaceWinner { vehicle_idx })
            }
            "PENA" => {
                let penalty_type = data[33];
                let infringement_type = data[34];
                let vehicle_idx = data[35];
                let other_vehicle_idx = data[36];
                let time = data[37];
                let lap_num = data[38];
                let places_gained = data[39];
                Event::Penalty(Penalty {
                    penalty_type,
                    infringement_type,
                    vehicle_idx,
                    other_vehicle_idx,
                    time,
                    lap_num,
                    places_gained,
                })
            }
            "SPTP" => {
                let vehicle_idx = data[33];
                let speed = LittleEndian::read_f32(&data[34..38]);
                let is_overall_fastest_in_session = data[38];
                let is_driver_fastest_in_session = data[39];
                let fastest_vehicle_idx_in_session = data[40];
                let fastest_speed_in_session = LittleEndian::read_f32(&data[41..45]);
                Event::SpeedTrap(SpeedTrap {
                    vehicle_idx,
                    speed,
                    is_overall_fastest_in_session,
                    is_driver_fastest_in_session,
                    fastest_vehicle_idx_in_session,
                    fastest_speed_in_session,
                })
            }
            "STLG" => {
                let num_lights = data[33];
                Event::StartLights(StartLights { num_lights })
            }
            "LGOT" => Event::LightsOut,
            "DTSV" => {
                let vehicle_idx = data[33];
                Event::DriveThroughPenaltyServed(DriveThroughPenaltyServed { vehicle_idx })
            }
            "SGSV" => {
                let vehicle_idx = data[33];
                Event::StopGoPenaltyServed(StopGoPenaltyServed { vehicle_idx })
            }
            "FLBK" => {
                let flashback_frame_id = LittleEndian::read_u32(&data[33..37]);
                let flashback_session_time = LittleEndian::read_f32(&data[37..41]);
                Event::Flashback(Flashback {
                    flashback_frame_id,
                    flashback_session_time,
                })
            }
            "BUTN" => {
                let button_status = LittleEndian::read_u32(&data[33..37]);
                Event::Buttons(Buttons { button_status })
            }
            "RDFL" => Event::RedFlag,
            "OVTK" => {
                let overtaking_vehicle_idx = data[33];
                let being_overtaken_vehicle_idx = data[34];
                Event::Overtake(Overtake {
                    overtaking_vehicle_idx,
                    being_overtaken_vehicle_idx,
                })
            }
            _ => panic!("Unknown event code: {}", event_string),
        };

        PacketEventData {
            header: header,
            event_string_code: event_string.to_string(),
            event,
        }
    }
}
