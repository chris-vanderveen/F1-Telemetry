use std::time::Instant;

pub struct PacketThroughput {
    session_id: u64,
    total_packet_size: i32, // Summed size of all recieved packets in bytes
    total_packets: u64,
    start_time: Instant,
    end_time: Instant,
    packets: Packets,
}

struct Packets {
    motion: i32,
    session: i32,
    lap_data: i32,
    event: i32,
    participants: i32,
    car_setups: i32,
    car_telemetry: i32,
    car_status: i32,
    final_classification: i32,
    lobby_info: i32,
    car_damage: i32,
    session_history: i32,
    tyre_sets: i32,
    motion_ex: i32,
}

impl Default for Packets {
    fn default() -> Self {
        Packets {
            motion: 0,
            session: 0,
            lap_data: 0,
            event: 0,
            participants: 0,
            car_setups: 0,
            car_telemetry: 0,
            car_status: 0,
            final_classification: 0,
            lobby_info: 0,
            car_damage: 0,
            session_history: 0,
            tyre_sets: 0,
            motion_ex: 0,
        }
    }
}

impl PacketThroughput {
    pub fn new(session_id: u64) -> Self {
        PacketThroughput {
            session_id,
            total_packet_size: 0,
            total_packets: 0,
            start_time: Instant::now(),
            end_time: Instant::now(),
            packets: Packets::default(),
        }
    }

    pub fn calculate_throughput(&self) -> Result<(), std::io::Error> {
        let session_length = self.end_time.duration_since(self.start_time).as_secs();
        println!(
            "Session Length: {}, Total Packets: {}, Total Size of Packets: {}",
            session_length, self.total_packets, self.total_packet_size,
        );
        Ok(())
    }

    pub fn update(&mut self, packet_id: &u8) -> Result<(), std::io::Error> {
        self.total_packets += 1;
        self.end_time = Instant::now();
        match packet_id {
            0 => {
                self.packets.motion += 1;
                self.total_packet_size += 1349;
            }
            1 => {
                self.packets.session += 1;
                self.total_packet_size += 644;
            }
            2 => {
                self.packets.lap_data += 1;
                self.total_packet_size += 1131;
            }
            3 => {
                self.packets.event += 1;
                self.total_packet_size += 45;
            }
            4 => {
                self.packets.participants += 1;
                self.total_packet_size += 1306;
            }
            5 => {
                self.packets.car_setups += 1;
                self.total_packet_size += 1107;
            }
            6 => {
                self.packets.car_telemetry += 1;
                self.total_packet_size += 1352;
            }
            7 => {
                self.packets.car_status += 1;
                self.total_packet_size += 1239;
            }
            8 => {
                self.packets.final_classification += 1;
                self.total_packet_size += 1020;
            }
            9 => {
                self.packets.lobby_info += 1;
                self.total_packet_size += 1218;
            }
            10 => {
                self.packets.car_damage += 1;
                self.total_packet_size += 953;
            }
            11 => {
                self.packets.session_history += 1;
                self.total_packet_size += 1460;
            }
            12 => {
                self.packets.tyre_sets += 1;
                self.total_packet_size += 231;
            }
            13 => {
                self.packets.motion_ex += 1;
                self.total_packet_size += 217;
            }
            _ => (),
        }
        Ok(())
    }
}
