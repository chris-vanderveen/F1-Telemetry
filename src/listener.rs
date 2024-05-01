use crate::packets::event::PacketEventData;
use crate::packets::header::PacketHeader;
use crate::packets::lap_data::PacketLapData;
use crate::packets::motion::PacketMotionData;
use crate::packets::participants::PacketParticipantsData;
use crate::packets::session::PacketSessionData;
use crate::udp_socket_interface::UdpSocketInterface;
use byteorder::{ByteOrder, LittleEndian};
use std::fs::OpenOptions;
use std::io::{Result, Write};

pub struct Listener<T: UdpSocketInterface> {
    pub socket: T,
}

impl<T: UdpSocketInterface> Listener<T> {
    pub fn new(port: u16) -> Result<Self> {
        let socket = T::bind(format!("10.0.0.120:{}", port))?;
        Ok(Listener { socket })
    }

    pub fn listen(&mut self) -> Result<()> {
        let mut buf = [0; 2048];
        loop {
            match self.socket.recv_from(&mut buf) {
                Ok((num_bytes, src_addr)) => {
                    println!("Recieved {} bytes from {}", num_bytes, src_addr);
                    self.process_packet(&buf[..num_bytes]);
                }
                Err(e) => {
                    eprintln!("Error receiving packet: {}", e);
                }
            }
        }
    }

    pub fn listen_once(&mut self) -> Result<()> {
        let mut buf = [0; 2048];
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open("packet_headers.txt")?;

        let mut i = 0;
        while i < 10000 {
            match self.socket.recv_from(&mut buf) {
                Ok((num_bytes, src_addr)) => {
                    println!("Received {} bytes from {}", num_bytes, src_addr);
                    let header_str = self.process_packet(&buf[..num_bytes]);
                    writeln!(file, "{:?}", header_str)?;
                    i += 1;
                }
                Err(e) => {
                    eprintln!("Error receiving packet: {}", e);
                    break; // Exit early on error
                }
            }
        }
        Ok(())
    }

    fn process_packet(&self, packet: &[u8]) {
        let packet_format = LittleEndian::read_u16(&packet[0..2]);
        let game_year = packet[2];
        let game_major_version = packet[3];
        let game_minor_version = packet[4];
        let packet_version = packet[5];
        let packet_id = packet[6];
        let session_uid = LittleEndian::read_u64(&packet[7..15]);
        let session_time = LittleEndian::read_f32(&packet[15..19]);
        let frame_identifier = LittleEndian::read_u32(&packet[19..23]);
        let overall_frame_identifier = LittleEndian::read_u32(&packet[23..27]);
        let player_car_index = packet[27];
        let secondary_player_car_index = packet[28];

        let packet_header = PacketHeader {
            packet_format,
            game_year,
            game_major_version,
            game_minor_version,
            packet_version,
            packet_id,
            session_uid,
            session_time,
            frame_identifier,
            overall_frame_identifier,
            player_car_index,
            secondary_player_car_index,
        };
    }
}
