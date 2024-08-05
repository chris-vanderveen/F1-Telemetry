use byteorder::{ByteOrder, LittleEndian};
use dotenv::dotenv;
use project::{
    data_processing::packet_throughput::PacketThroughput,
    db::{
        client::{self, create_local_client},
        tables,
    },
    listener::Listener,
    packet::{Packet, SerializeToJSON},
    packets::{
        car_damage_data::PacketCarDamageData, car_setups::PacketCarSetupData,
        car_status::PacketCarStatusData, car_telemetry::PacketCarTelemetryData,
        event::PacketEventData, final_classication::PacketFinalClassification,
        lap_data::PacketLapData, lobby_info::PacketLobbyInfoData, motion::PacketMotionData,
        motion_ex::PacketMotionExData, participants::PacketParticipantsData,
        session::PacketSessionData, session_history::PacketSessionHistoryData,
        tyre_sets::PacketTyreSetData,
    },
};
use std::{env, net::UdpSocket};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();

    // DB initialization
    let client = create_local_client().await;

    let bind_address = env::var("UDP_ADDRESS").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port_str = env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let port: u16 = port_str.parse().expect("PORT must be a number");
    let full_bind_address = format!("{}:{}", bind_address, port);

    let listener_result = Listener::<UdpSocket>::new(&full_bind_address);

    match listener_result {
        Ok(mut listener) => {
            let mut throughput = PacketThroughput::new();

            listener.listen(|packet_data| {
                let packet_id = packet_data[6];
                let session_id = LittleEndian::read_u64(&packet_data[7..15]);

                // Update session ID if there is a change
                if throughput.session_id() != session_id && session_id != 0 {
                    throughput.set_session_id(session_id);
                }

                // If in a current session
                if session_id != 0 {
                    if let Err(e) = throughput.update(&packet_id) {
                        eprintln!("Error updating throughput: {}", e);
                    }

                    match packet_id {
                        0 => {
                            let motion_packet =
                                Packet::Motion(PacketMotionData::from_bytes(&packet_data));
                            if let Err(e) = motion_packet.serialize_to_json() {
                                eprintln!("Error serializing motion packet to JSON: {}", e);
                            }
                        }
                        1 => {
                            let session_packet =
                                Packet::Session(PacketSessionData::from_bytes(&packet_data));
                            if let Err(e) = session_packet.serialize_to_json() {
                                eprintln!("Error serializing session packet to JSON: {}", e);
                            }
                        }
                        2 => {
                            let lap_data_packet =
                                Packet::LapData(PacketLapData::from_bytes(&packet_data));
                            if let Err(e) = lap_data_packet.serialize_to_json() {
                                eprintln!("Error serializing lap data packet to JSON: {}", e);
                            }
                        }
                        3 => {
                            let event_data_packet =
                                Packet::Event(PacketEventData::from_bytes(&packet_data));
                            if let Err(e) = event_data_packet.serialize_to_json() {
                                eprintln!("Error serializing event data packet to JSON: {}", e);
                            }
                        }
                        4 => {
                            let participants_packet = Packet::Participants(
                                PacketParticipantsData::from_bytes(&packet_data),
                            );
                            if let Err(e) = participants_packet.serialize_to_json() {
                                eprintln!("Error serializing participants packet to JSON: {}", e);
                            }
                        }
                        5 => {
                            let car_setups_packet =
                                Packet::Setups(PacketCarSetupData::from_bytes(&packet_data));
                            if let Err(e) = car_setups_packet.serialize_to_json() {
                                eprintln!("Error serializing car setups packet to JSON: {}", e);
                            }
                        }
                        6 => {
                            let car_telemetry_packet =
                                Packet::Telemetry(PacketCarTelemetryData::from_bytes(&packet_data));
                            if let Err(e) = car_telemetry_packet.serialize_to_json() {
                                eprintln!("Error serializing car telemetry packet to JSON: {}", e);
                            }
                        }
                        7 => {
                            let car_status_packet =
                                Packet::Status(PacketCarStatusData::from_bytes(&packet_data));
                            if let Err(e) = car_status_packet.serialize_to_json() {
                                eprintln!("Error serializing car status packet to JSON: {}", e);
                            }
                        }
                        8 => {
                            let final_classification_packet = Packet::FinalClassification(
                                PacketFinalClassification::from_bytes(&packet_data),
                            );
                            if let Err(e) = final_classification_packet.serialize_to_json() {
                                eprintln!(
                                    "Error serializing final classification packet to JSON: {}",
                                    e
                                );
                            }
                            // When a final classification is encountered calculate throughput
                            // and return false to end the listener loop
                            if let Err(e) = throughput.calculate_throughput() {
                                eprintln!("Error calculating throughput: {}", e);
                            }
                            return false;
                        }
                        9 => {
                            let lobby_info_packet =
                                Packet::Lobby(PacketLobbyInfoData::from_bytes(&packet_data));
                            if let Err(e) = lobby_info_packet.serialize_to_json() {
                                eprintln!("Error serializing lobby info packet to JSON: {}", e);
                            }
                        }
                        10 => {
                            let car_damage_packet =
                                Packet::Damage(PacketCarDamageData::from_bytes(&packet_data));
                            if let Err(e) = car_damage_packet.serialize_to_json() {
                                eprintln!("Error serializing car damage packet to JSON: {}", e);
                            }
                        }
                        11 => {
                            let session_history_packet =
                                Packet::History(PacketSessionHistoryData::from_bytes(&packet_data));
                            if let Err(e) = session_history_packet.serialize_to_json() {
                                eprintln!(
                                    "Error serializing session history packet to JSON: {}",
                                    e
                                );
                            }
                        }
                        12 => {
                            let tyre_sets_packet =
                                Packet::TyreSets(PacketTyreSetData::from_bytes(&packet_data));
                            if let Err(e) = tyre_sets_packet.serialize_to_json() {
                                eprintln!("Error serializing tyre set packet to json: {}", e);
                            }
                        }
                        13 => {
                            let motion_ex_packet =
                                Packet::MotionEx(PacketMotionExData::from_bytes(&packet_data));
                            if let Err(e) = motion_ex_packet.serialize_to_json() {
                                eprintln!("Error serializing motion ex packet to JSON: {}", e);
                            }
                        }
                        _ => {
                            eprintln!("Unknown packet id: {}", packet_id);
                        }
                    }; // End of match statement
                }
                true
            })?;
        }
        Err(e) => {
            eprintln!("Failed to create listener: {}", e);
        }
    }
    Ok(())
}
