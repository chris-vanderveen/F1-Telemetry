use std::io::Result;
use std::net::UdpSocket;

pub trait UdpSocketInterface {
    fn bind(address: String) -> Result<Self>
    where
        Self: Sized;
    fn recv_from(&self, buf: &mut [u8]) -> Result<(usize, std::net::SocketAddr)>;
}

impl UdpSocketInterface for UdpSocket {
    fn bind(address: String) -> Result<Self> {
        UdpSocket::bind(address)
    }

    fn recv_from(&self, buf: &mut [u8]) -> Result<(usize, std::net::SocketAddr)> {
        self.recv_from(buf)
    }
}
