use project::packet::PacketHeader;
use project::udp_socket_interface::UdpSocketInterface;
use std::io::Result;
use std::net::SocketAddr;

pub struct MockUdpSocket {
    pub packet: Vec<u8>,
}

impl UdpSocketInterface for MockUdpSocket {
    fn bind(_address: String) -> Result<Self> {
        let packet = PacketHeader::new().as_bytes();
        Ok(MockUdpSocket { packet })
    }

    fn recv_from(&self, buf: &mut [u8]) -> Result<(usize, SocketAddr)> {
        let len = self.packet.len().min(buf.len());
        buf[..len].copy_from_slice(&self.packet[..len]);
        Ok((len, "10.0.0.120:20777".parse().unwrap()))
    }
}
