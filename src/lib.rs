pub mod listener;
pub mod packet;
pub mod udp_socket_interface;

pub mod constants {
    pub const PACKET_HEADER_SIZE: usize = 29;
}

pub mod packets {
    pub mod car_damage_data;
    pub mod car_setups;
    pub mod car_status;
    pub mod car_telemetry;
    pub mod event;
    pub mod final_classication;
    pub mod header;
    pub mod lap_data;
    pub mod lobby_info;
    pub mod motion;
    pub mod participants;
    pub mod session;
    pub mod session_history;
    pub mod tyre_sets;
}
