use byteorder::{ByteOrder, LittleEndian};
use serde::{Deserialize, Serialize};

// 29 bytes
#[derive(Debug, Serialize, Deserialize)]
pub struct PacketHeader {
    // Packet format for game year (2023)
    pub packet_format: u16,
    // Last two digits of game year (23)
    pub game_year: u8,
    // Game major version "x.00"
    pub game_major_version: u8,
    // Game minor version "1.xx"
    pub game_minor_version: u8,
    // Version of packet type
    pub packet_version: u8,
    // Packet type (0-13)
    pub packet_id: u8,
    // Unique session id
    pub session_uid: u64,
    // Time in session in ms
    pub session_time: f32,
    // Frame data was recieved from
    pub frame_identifier: u32,
    // Overall identifier for the frame the data was retrieved on, is not reset by flashbacks
    pub overall_frame_identifier: u32,
    // Index of players car in the array
    pub player_car_index: u8,
    // Index of second players car in the array. 255 if no second player
    pub secondary_player_car_index: u8,
}

impl PacketHeader {
    pub fn from_bytes(packet: &[u8]) -> Self {
        PacketHeader {
            packet_format: LittleEndian::read_u16(&packet[0..2]),
            game_year: packet[2],
            game_major_version: packet[3],
            game_minor_version: packet[4],
            packet_version: packet[5],
            packet_id: packet[6],
            session_uid: LittleEndian::read_u64(&packet[7..15]),
            session_time: LittleEndian::read_f32(&packet[15..19]),
            frame_identifier: LittleEndian::read_u32(&packet[19..23]),
            overall_frame_identifier: LittleEndian::read_u32(&packet[23..27]),
            player_car_index: packet[27],
            secondary_player_car_index: packet[28],
        }
    }
}
