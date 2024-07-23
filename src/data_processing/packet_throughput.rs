use std::time::Instant;

pub struct PacketThroughput {
    session_id: u64,
    total_packet_size: u64, // Summed size of all recieved packets in bytes
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
    pub fn new() -> Self {
        PacketThroughput {
            session_id: 0,
            total_packet_size: 0,
            total_packets: 0,
            start_time: Instant::now(),
            end_time: Instant::now(),
            packets: Packets::default(),
        }
    }

    pub fn session_id(&self) -> u64 {
        self.session_id
    }

    pub fn set_session_id(&mut self, id: u64) {
        if self.session_id() == 0 && id != 0 {
            self.session_id = id;
            self.start_time = Instant::now();
        }
    }

    pub fn calculate_throughput(&mut self) -> Result<(), std::io::Error> {
        self.end_time = Instant::now();
        let session_length = self.end_time.duration_since(self.start_time).as_secs_f64();
        println!(
            "Session Length: {:.2}s, Total Packets: {}, Total Size of Packets: {} bytes",
            session_length, self.total_packets, self.total_packet_size,
        );
        println!("Packet Breakdown:");
        println!("  Motion: {}", self.packets.motion);
        println!("  Session: {}", self.packets.session);
        println!("  Lap Data: {}", self.packets.lap_data);
        println!("  Event: {}", self.packets.event);
        println!("  Participants: {}", self.packets.participants);
        println!("  Car Setups: {}", self.packets.car_setups);
        println!("  Car Telemetry: {}", self.packets.car_telemetry);
        println!("  Car Status: {}", self.packets.car_status);
        println!(
            "  Final Classification: {}",
            self.packets.final_classification
        );
        println!("  Lobby Info: {}", self.packets.lobby_info);
        println!("  Car Damage: {}", self.packets.car_damage);
        println!("  Session History: {}", self.packets.session_history);
        println!("  Tyre Sets: {}", self.packets.tyre_sets);
        println!("  Motion Ex: {}", self.packets.motion_ex);

        let throughput = (self.total_packet_size as f64) / session_length;
        println!("Throughput: {:.2} bytes/second", throughput);
        Ok(())
    }

    pub fn update(&mut self, packet_id: &u8) -> Result<(), std::io::Error> {
        self.total_packets += 1;

        match packet_id {
            0 => {
                self.packets.motion += 1;
                self.total_packet_size += 1378; // 1349 + sizeof(header(29 bytes))
            }
            1 => {
                self.packets.session += 1;
                self.total_packet_size += 673;
            }
            2 => {
                self.packets.lap_data += 1;
                self.total_packet_size += 1160;
            }
            3 => {
                self.packets.event += 1;
                self.total_packet_size += 74;
            }
            4 => {
                self.packets.participants += 1;
                self.total_packet_size += 1335;
            }
            5 => {
                self.packets.car_setups += 1;
                self.total_packet_size += 1136;
            }
            6 => {
                self.packets.car_telemetry += 1;
                self.total_packet_size += 1381;
            }
            7 => {
                self.packets.car_status += 1;
                self.total_packet_size += 1268;
            }
            8 => {
                self.packets.final_classification += 1;
                self.total_packet_size += 1049;
            }
            9 => {
                self.packets.lobby_info += 1;
                self.total_packet_size += 1247;
            }
            10 => {
                self.packets.car_damage += 1;
                self.total_packet_size += 982;
            }
            11 => {
                self.packets.session_history += 1;
                self.total_packet_size += 1489;
            }
            12 => {
                self.packets.tyre_sets += 1;
                self.total_packet_size += 260;
            }
            13 => {
                self.packets.motion_ex += 1;
                self.total_packet_size += 246;
            }
            _ => (),
        }
        Ok(())
    }
}
