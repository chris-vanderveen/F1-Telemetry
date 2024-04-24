mod mocks;

use mocks::MockUdpSocket;
use project::listener::Listener;

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;

    #[test]
    fn test_listener_recieves_data() {
        let data = b"Test data".to_vec();
        let mock_socket = mocks::MockUdpSocket { packet: data };
        let mut listener = Listener {
            socket: mock_socket,
        };

        assert!(listener.listen_once().is_ok());
    }
}
