use crate::packets::header::PacketHeader;
use byteorder::{ByteOrder, LittleEndian};

// 50 bytes * 22 =
#[derive(Debug)]
pub struct LapData {
    last_lap_time_ms: u32,
    current_lap_time_ms: u32,
    sector_1_time_ms: u16,
    sector_1_time_mins: u8,
    sector_2_time_ms: u16,
    sector_2_time_mins: u8,
    delta_to_car_ahead_ms: u16,
    delta_to_race_leader: u16,
    lap_distance: f32,   // current distance in meters (vehicle)
    total_distance: f32, // Total distance travelled in session
    safety_car_delta: f32,
    car_postion: u8,
    current_lap_number: u8,
    pit_status: u8,              // 0 = none, 1 = pitting, 2 = in pit area
    num_pit_stops: u8,           // Number of pit stops taken in this race
    sector: u8,                  // 1, 2, 3
    current_lap_invalid: u8,     // 0 = valid, 1 = invalid
    penalties: u8,               // Accumulated time penalties in (s)
    total_warnings: u8,          // Total number of warnings issued
    corner_cutting_warnings: u8, // Accumulated corner-cut warnings
    num_unserved_drive_through_pens: u8,
    num_unserved_stop_go_pens: u8,
    grid_position: u8, // Grid pos race was started in
    // Status of driver - 0 = in garage, 1 = flying lap
    // 2 = in lap, 3 = out lap, 4 = on track
    driver_status: u8,
    // Result status - 0 = invalid, 1 = inactive, 2 = active
    // 3 = finished, 4 = didnotfinish, 5 = disqualified
    // 6 = not classified, 7 = retired
    result_status: u8,
    pit_lane_timer_active: u8, // 0 = inactive, 1 = active
    pit_lane_time_in_lane_ms: u16,
    pit_stop_time_ms: u16, // Time of actual pit stop in ms
    pit_stop_should_serve_pen: u8,
}

// Size: 1131 bytes
#[derive(Debug)]
pub struct PacketLapData {
    header: PacketHeader,
    lap_data: Vec<LapData>,    // There is a maximum of 22
    time_trial_pb_car_idx: u8, // index of pb car in time trial (255 if invalid)
    time_trial_rival_car_idx: u8,
}

impl PacketLapData {
    pub fn from_bytes(data: &[u8]) -> Self {
        let packet_header = PacketHeader::from_bytes(&data[0..29]);
        let mut lap_data = Vec::new();

        let mut offset = 50;
        for i in 0..22 {
            let lap = LapData::from_bytes(&data[offset..offset + 50]);
            lap_data.push(lap);
            offset += 5;
        }

        PacketLapData {
            header: packet_header,
            lap_data: lap_data,
            time_trial_pb_car_idx: data[1129],
            time_trial_rival_car_idx: data[1130],
        }
    }
}

impl LapData {
    fn from_bytes(data: &[u8]) -> Self {
        LapData {
            last_lap_time_ms: LittleEndian::read_u32(&data[0..4]),
            current_lap_time_ms: LittleEndian::read_u32(&data[4..8]),
            sector_1_time_ms: LittleEndian::read_u16(&data[8..10]),
            sector_1_time_mins: data[10],
            sector_2_time_ms: LittleEndian::read_u16(&data[11..13]),
            sector_2_time_mins: data[13],
            delta_to_car_ahead_ms: LittleEndian::read_u16(&data[14..16]),
            delta_to_race_leader: LittleEndian::read_u16(&data[16..18]),
            lap_distance: LittleEndian::read_f32(&data[18..22]),
            total_distance: LittleEndian::read_f32(&data[22..26]),
            safety_car_delta: LittleEndian::read_f32(&data[26..30]),
            car_postion: data[30],
            current_lap_number: data[31],
            pit_status: data[32],
            num_pit_stops: data[33],
            sector: data[34],
            current_lap_invalid: data[35],
            penalties: data[36],
            total_warnings: data[37],
            corner_cutting_warnings: data[38],
            num_unserved_drive_through_pens: data[39],
            num_unserved_stop_go_pens: data[40],
            grid_position: data[41],
            driver_status: data[42],
            result_status: data[43],
            pit_lane_timer_active: data[44],
            pit_lane_time_in_lane_ms: LittleEndian::read_u16(&data[45..47]),
            pit_stop_time_ms: LittleEndian::read_u16(&data[47..49]),
            pit_stop_should_serve_pen: data[49],
        }
    }
}
