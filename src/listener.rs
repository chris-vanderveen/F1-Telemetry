use super::udp_socket_interface::UdpSocketInterface;
use std::io::Result;

pub struct Listener<T: UdpSocketInterface> {
    socket: T,
}

impl<T: UdpSocketInterface> Listener<T> {
    pub fn new(port: u16) -> Result<Self> {
        let socket = T::bind(format!("127.0.0.1:{}", port))?;
        Ok(Listener { socket })
    }

    pub fn listen(&mut self) -> Result<()> {
        let mut buf = [0; 1400];
        loop {
            let (num_bytes, src_addr) = self.socket.recv_from(&mut buf)?;
            println!("Received {} bytes from {}", num_bytes, src_addr);
            self.process_packet(&buf[..num_bytes]);

            // Example break condition or method to stop could be added here
        }
    }

    fn process_packet(&self, packet: &[u8]) {
        println!("Processing packet: {:?}", packet);
    }
}
