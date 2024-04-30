use crate::packets::header::PacketHeader;
use crate::packets::participants::Nationality;
use crate::packets::participants::Team;

// Two every second while in lobby
// 1218 bytes
#[derive(Debug)]
pub struct LobbyInfoData {
    ai_controlled: u8, // 1 = ai, 2 = human
    team_id: Team,
    nationality: Nationality,
    platform: u8,
    name: Vec<char>,
    car_number: u8,
    ready_status: u8,
}

#[derive(Debug)]
pub struct PacketLobbyInfoData {
    header: PacketHeader,
    num_players: u8,
    lobby_info_data: LobbyInfoData,
}
