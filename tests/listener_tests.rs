#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_listener_recieves_data() {
        let mut listener = Listener {
            socket: mocks::MockUdpSocket {},
        };
        assert!(listener.listen().is_ok());
    }
}
