use project::{
    listener::Listener,
    packet::{Packet, SerializeToJSON},
    packets::{motion::PacketMotionData, session::PacketSessionData},
};
use std::net::UdpSocket;

fn main() -> Result<(), std::io::Error> {
    let listener_result = Listener::<UdpSocket>::new(20777);

    match listener_result {
        Ok(mut listener) => {
            listener.listen(|packet_data| {
                let packet_id = packet_data[6];
                match packet_id {
                    0 => {
                        let motion_packet = PacketMotionData::from_bytes(&packet_data);
                        let _ = motion_packet.serialize_to_json(); // This needs to be a `Packet`.serialize_to_json()
                    }
                    1 => {
                        let session_packet = PacketSessionData::from_bytes(&packet_data);
                    }
                    _ => {
                        eprintln!("Unknown packet id: {}", packet_id);
                    }
                };
            })?;
        }
        Err(e) => {
            eprintln!("Failed to create listener: {}", e);
        }
    }

    Ok(())
}
