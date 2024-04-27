#[derive(Debug)]
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
