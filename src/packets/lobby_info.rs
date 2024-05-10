use crate::packets::header::PacketHeader;
use serde::{Deserialize, Serialize};

// Two every second while in lobby
// 54 bytes
#[derive(Debug, Serialize, Deserialize)]
pub struct LobbyInfoData {
    ai_controlled: u8, // 1 = ai, 2 = human
    team_id: u8,
    nationality: u8,
    platform: u8,
    name: String,
    car_number: u8,
    ready_status: u8,
}

// 1218 bytes
#[derive(Debug, Serialize, Deserialize)]
pub struct PacketLobbyInfoData {
    pub header: PacketHeader,
    num_players: u8,
    lobby_info_data: Vec<LobbyInfoData>,
}

impl PacketLobbyInfoData {
    pub fn from_bytes(data: &[u8]) -> Self {
        let header = PacketHeader::from_bytes(&data[0..29]);
        let num_players = data[29];
        let mut lobby_info_data = Vec::new();
        let mut offset = 30;

        for _i in 0..num_players {
            let lobby_info = LobbyInfoData::from_bytes(&data[offset..offset + 54]);
            lobby_info_data.push(lobby_info);
            offset += 54;
        }

        PacketLobbyInfoData {
            header: header,
            num_players: num_players,
            lobby_info_data: lobby_info_data,
        }
    }
}

impl LobbyInfoData {
    fn from_bytes(data: &[u8]) -> Self {
        let name_bytes = &data[4..52];
        let name = std::str::from_utf8(name_bytes)
            .unwrap_or_default()
            .trim_end_matches(char::from(0))
            .to_string();

        LobbyInfoData {
            ai_controlled: data[0],
            team_id: data[1],
            nationality: data[2],
            platform: data[3],
            name: name,
            car_number: data[52],
            ready_status: data[53],
        }
    }
}
