use std::mem::size_of;

#[repr(C, packed)]
pub struct PacketHeader {
    packet_format: u16,
    game_year: u8,
    game_major_version: u8,
    game_minor_version: u8,
    packet_version: u8,
    packet_id: u8,
    session_uid: u64,
    session_time: f32,
    frame_identifier: u32,
    overall_frame_identifier: u32,
    player_car_index: u8,
    secondary_player_car_index: u8,
}

impl PacketHeader {
    pub fn new() -> PacketHeader {
        PacketHeader {
            packet_format: 2023,
            game_year: 23,
            game_major_version: 1,
            game_minor_version: 0,
            packet_version: 1,
            packet_id: 1,
            session_uid: 1234567890123456789,
            session_time: 1234.5,
            frame_identifier: 1000,
            overall_frame_identifier: 2000,
            player_car_index: 1,
            secondary_player_car_index: 255,
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let size = size_of::<Self>();
        let ptr = self as *const Self as *const u8;
        unsafe { Vec::from_raw_parts(ptr as *mut u8, size, size) }
    }
}
