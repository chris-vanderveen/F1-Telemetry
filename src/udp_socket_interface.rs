use std::io::Result;
use std::net::UdpSocket;

pub trait UdpSocketInterface {
    fn bind(address: String) -> Result<Self>
    where
        Self: Sized;

    fn recv_from(&self, buf: &mut [u8]) -> Result<(usize, String)>;
}

impl UdpSocketInterface for UdpSocket {
    fn bind(address: String) -> Result<Self> {
        UdpSocket::bind(address).map_err(|e| e.into())
    }

    fn recv_from(&self, buf: &mut [u8]) -> Result<(usize, String)> {
        self.recv_from(buf)
            .map(|(size, addr)| (size, addr.to_string()))
    }
}
