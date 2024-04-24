use std::mem::size_of;

#[repr(C, packed)]
#[derive(Debug)]
pub struct PacketHeader {
    pub packet_format: u16,
    pub game_year: u8,
    pub game_major_version: u8,
    pub game_minor_version: u8,
    pub packet_version: u8,
    pub packet_id: u8,
    pub session_uid: u64,
    pub session_time: f32,
    pub frame_identifier: u32,
    pub overall_frame_identifier: u32,
    pub player_car_index: u8,
    pub secondary_player_car_index: u8,
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
        let mut buffer = vec![0u8; size];
        let ptr = buffer.as_mut_ptr() as *mut Self;
        unsafe {
            // Copy data into the buffer
            ptr.copy_from_nonoverlapping(self, 1);
        }
        buffer
    }
}
