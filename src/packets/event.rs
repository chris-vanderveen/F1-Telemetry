use serde::Serialize;

use crate::packets::header::PacketHeader;

// The Event details packet is different for each type of event
// Make sure the correct type is interpreted.
#[derive(Debug, Serialize)]
pub struct FastestLap {
    vehicle_idx: u8, // Index of car achieving fastest lap
    lap_time: f32,   // Lap time in seconds
}

#[derive(Debug, Serialize)]
pub struct Retirement {
    vehicle_idx: u8, // Index of vehicle retiring
}

#[derive(Debug, Serialize)]
pub struct TeamMateInPits {
    vehicle_idx: u8, // Vehicle index of teammate
}

#[derive(Debug, Serialize)]
pub struct RaceWinner {
    vehicle_idx: u8, // Race winner
}

#[derive(Debug, Serialize)]
pub struct Penalty {
    penalty_type: u8,
    infringement_type: u8,
    vehicle_idx: u8,       // Index of the car pen is applied to
    other_vehicle_idx: u8, // Index of other vehicle involved
    time: u8,              // Time gained or spent doing action
    lap_num: u8,           // Lap penalty occurred on
    places_gained: u8,     // Num places gained doing this
}

#[derive(Debug, Serialize)]
pub struct SpeedTrap {
    vehicle_idx: u8,                    // Index of vehicle triggering speed trap
    speed: f32,                         // Top speed achieved (km/h)
    is_overall_fastest_in_session: u8,  // true = 1, false = 0
    is_driver_fastest_in_session: u8,   // true = 1, false = 0
    fastest_vehicle_idx_in_session: u8, // Vehicle index of fastest vehicle in session
    fastest_speed_in_session: f32,      // Highest speed of fastest vehicle in session
}

#[derive(Debug, Serialize)]
pub struct StartLights {
    num_lights: u8, // Number of lights showing
}

#[derive(Debug, Serialize)]
pub struct DriveThroughPenaltyServed {
    vehicle_idx: u8, // Index of vehicle serving drive through
}

#[derive(Debug, Serialize)]
pub struct StopGoPenaltyServed {
    vehicle_idx: u8, // Index of vehicle serving stop and go
}

#[derive(Debug, Serialize)]
pub struct Flashback {
    flashback_frame_id: u32,     // Frame flashed back to
    flashback_session_time: f32, // Session time flashed to
}

#[derive(Debug, Serialize)]
pub struct Buttons {
    button_status: u32, // Bit flags specifying which buttons are being pressed
}

#[derive(Debug, Serialize)]
pub struct Overtake {
    overtaking_vehicle_idx: u8,      // Index of overtaking vehicle
    being_overtaken_vehicle_idx: u8, // Index of vehicle being overtaken
}

#[derive(Debug, Serialize)]
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
    LightsOut,
    DriveThroughPenaltyServed(DriveThroughPenaltyServed),
    StopGoPenaltyServed(StopGoPenaltyServed),
    Flashback(Flashback),
    Overtake(Overtake),
}

#[derive(Debug, Serialize)]
pub struct PacketEventData {
    packet_header: PacketHeader,
    event_string_code: Vec<u8>,
    event: Event,
}

// Not sure if I actually need this or not
#[derive(Debug)]
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
#[derive(Debug)]
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
#[derive(Debug)]
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
