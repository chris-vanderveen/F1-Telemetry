use crate::constants::PACKET_HEADER_SIZE;
use crate::packet::Packet;
use crate::packets::header::PacketHeader;
use crate::udp_socket_interface::UdpSocketInterface;
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
        match PacketHeader::from_bytes(packet) {
            Ok(header) => {
                let packet_data = &packet[PACKET_HEADER_SIZE..];
                let packet_instance = Packet::new(header, packet_data);
                packet_instance.process();
            }
            Err(e) => eprintln!("Failed to parse packet header: {}", e),
        }
    }
}
