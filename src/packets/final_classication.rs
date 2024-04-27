// Once at the end of race
#[derive(Debug)]
pub struct FinalClassificationData {
    position: u8,      // Finishing pos
    num_laps: u8,      // NUmber of laps completed
    grid_position: u8, // Grid position of car
    points: u8,        // number of points scored
    num_pit_stops: u8,
    results_status: u8,   // 0 = invalid, 1 = inactive, 2 = active
    best_lap_time: u32,   // (Ms)
    total_race_time: u32, // (ms)
    penalties_time: u8,   // Accumulation of penalty time (s)
    num_penalties: u8,
    num_tyre_stints: u8,
    tyre_stints_actual: vec<TyreCompund>,
    tyre_stints_visual: vec<TyreCompund>,
    tyre_stints_end_lap: vec<TyreCompund>,
}

#[derive(Debug)]
pub struct PacketFinalClassification {
    header: PacketHeader,
    num_cars: u8,
    final_classification_data: FinalClassificationData,
}
