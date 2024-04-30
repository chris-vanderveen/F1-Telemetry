use crate::packet::TyreCompound;
use crate::packets::header::PacketHeader;

// Frequency: 20/s cycles through cars
// Size: 231 bytes
#[derive(Debug)]
pub struct TyreSetData {
    actual_tyre_compound: TyreCompound,
    visual_tyre_compound: TyreCompound,
    wear: u8, // Tyre wear (Percentage)
    available: u8,
    recommended_session: u8,
    life_span: u8,       // laps left in this tyre set
    usable_life: u8,     // Max number of recommended laps
    lap_delta_time: i16, // delta lap time (ms) compared to currently fitted tyre
    fitted: u8,          // Whether the set is fitted or not
}

#[derive(Debug)]
pub struct PacketTyreSetData {
    header: PacketHeader,
    car_idx: u8,
    tyre_set_data: Vec<TyreSetData>, // 20 total, 13 dry + 7 wet
    fitted_idx: u8,                  // Index into vec of fitted tyre
}
