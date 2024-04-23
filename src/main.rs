mod listener;
mod udp_socket_interface;

use listener::Listener;

fn main() {
    let port = 20777;
    let listener = Listener::<std::net::UdpSocket>::new(port);
    match listener {
        Ok(mut l) => {
            println!("Listener Started on port {}", port);
        }
        Err(e) => {
            eprintln!("Failed to start listener {}", e);
        }
    }
}
