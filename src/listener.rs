/// A generic network listener for receiving data packets over UDP.
///
/// The `Listener` struct is generic over `T` where `T` must implement the `UdpSocketInterface`.
/// This allows the listener to be used with any UDP socket implementation that adheres to this interface,
/// making the listener flexible and adaptable to different socket implementations.
use crate::udp_socket_interface::UdpSocketInterface;
use log::{error, info};
use std::io::Result;

pub struct Listener<T: UdpSocketInterface> {
    /// The socket used for receiving data.
    pub socket: T,
}

impl<T: UdpSocketInterface> Listener<T> {
    /// Constructs a new `Listener` with a UDP socket bound to the specified port.
    ///
    /// # Arguments
    ///
    /// * `port` - The port number to bind the UDP socket to.
    ///
    /// # Returns
    ///
    /// Returns `Ok(Listener)` if the socket is successfully bound,
    /// otherwise returns an `Err` containing the io error.
    ///
    /// # Errors
    ///
    /// This function will return an error if the socket fails to bind to the specified port.
    /// The error returned is an `io::Error` which can be queried to determine the specific problem.
    pub fn new(address: &str) -> Result<Self> {
        let socket = T::bind(address.to_string())?;
        Ok(Listener { socket })
    }

    /// Starts listening for packets on the bound socket. Received packets are passed to the provided callback function.
    ///
    /// This method will continuously receive data from the socket and will pass the data slices to
    /// the provided `process_packet` function. The loop continues indefinitely unless a receiving error occurs.
    ///
    /// # Arguments
    ///
    /// * `process_packet` - A mutable callback function that takes a slice of bytes (the packet data).
    ///   The function should handle processing of the packet data. The function does not return a value.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the listening loop ends normally (which in this setup, only ends on error),
    /// otherwise returns an `Err` containing the io error that caused the listening loop to terminate.
    ///
    /// # Errors
    ///
    /// This function will return an error if there's an issue receiving data from the socket,
    /// such as a network error or socket closure.
    pub fn listen<F>(&mut self, mut process_packet: F) -> Result<()>
    where
        F: FnMut(&[u8]) -> bool,
    {
        let mut buf = [0; 2048];
        loop {
            match self.socket.recv_from(&mut buf) {
                Ok((num_bytes, src_addr)) => {
                    info!("Received {} bytes from {}", num_bytes, src_addr);
                    if !process_packet(&buf[..num_bytes]) {
                        ()
                    }
                }
                Err(e) => {
                    error!("Error receiving packet: {}", e);
                    return Err(e);
                }
            }
        }
    }
}
