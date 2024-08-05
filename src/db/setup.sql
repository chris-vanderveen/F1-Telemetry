-- Session Types
create type session_type as enum (
    'unknown',
    'p1',
    'p2',
    'p3',
    'short_practice',
    'q1',
    'q2',
    'q3',
    'short_qauli',
    'osq',
    'race',
    'race2',
    'race3',
    'time_trial'
);

-- Session Status
create type session_status as enum (
    'not_started',
    'in_progress',
    'completed',
    'abandoned'
);

-- Weather types
create type weather_condition as enum (
    'clear',
    'light_cloud',
    'overcast',
    'light_rain',
    'heavy_rain',
    'storm'
);

-- Event types
create type event_type as enum (
    'session_started',
    'session_ended',
    'fastest_lap',
    'retirement',
    'drs_enabled',
    'drs_disabled',
    'team_mate_in_pits',
    'chequered_flag',
    'race_winner',
    'penalty_issued',
    'speed_trap_triggered',
    'start_lights',
    'lights_out',
    'drive_through_served',
    'stop_go_served',
    'flashback',
    'button_status',
    'red_flag',
    'overtake'
);

-- Tyre compounds
create type tyre_compound as enum (
    'c5',
    'c4',
    'c3',
    'c2',
    'c1',
    'super_soft',
    'soft',
    'medium',
    'hard',
    'intermediate',
    'wet'
);

-- ERS deployment modes
create type ers_mode as enum (
    'none',
    'medium',
    'overtake',
    'hotlap'
);

-- Definition of users for later
create table if not exists users (
    id serial primary key,
    username varchar(255) not null,
    password varchar(255) not null,
    created_at timestamp default now()
);

create table if not exists user_sessions (
    user_id int references users(id),
    session_id int references sessions(session_id),
    primary key (user_id, session_id)
);

-- Sessions
-- Defines *static* data for a session
create table if not exists sessions (
    session_id serial primary key,
    user_id int references users(id),
    session_name varchar(255) not null,
    session_start timestamp not null,
    session_end timestamp,
    session_type session_type not null,
    session_status smallint not null, -- completed, in_progress, etc.
    forecast_accuracy smallint not null,
    track_id smallint not null,
    total_laps smallint,
    track_length int not null,
    pit_speed_limit smallint,
    session_duration int not null,
    num_marshal_zones smallint not null,
    network_game smallint not null,
    ai_difficulty smallint,
    steering_assist smallint,
    braking_assist smallint,
    gearbox_assist smallint,
    pit_assist smallint,
    pit_release_assist smallint,
    ers_assist smallint,
    drs_assist smallint,
    game_mode smallint,
    rule_set smallint,
    time_of_day smallint,
    session_length smallint,
    speed_units smallint,
    temperature_units smallint
);

-- Session Data
-- defines dynamic data for a session, i.e. multiple/session
create table if not exists session_data (
    session_id int references sessions(session_id),
    session_data_id serial primary key,
    timestamp timestamp default now(),
    weather smallint,
    track_temp smallint,
    air_temp smallint,
    session_time_remaining int,
    safety_car_status smallint,
    num_weather_samples smallint,
    pit_window_ideal_lap smallint,
    pit_window_latest_lap smallint,
    pit_stop_rejoin_position smallint,
    steering_assist smallint
);

create table if not exists motion_data (
    session_id int references sessions(session_id),
    motion_id serial primary key,
    car_id int,
    timestamp timestamp not null,
    world_pos_x float not null,
    world_pos_y float not null,
    world_pos_z float not null,
    world_vel_x float not null,
    world_vel_y float not null,
    world_vel_z float not null,
    world_forward_dir_x float not null,
    world_forward_dir_y float not null,
    world_forward_dir_z float not null,
    g_force_lat float not null,
    g_force_long float not null,
    g_force_vert float not null,
    yaw float not null,
    pitch float not null,
    roll float not null
);

create table if not exists weather_sample (
    weather_sample_id serial primary key,
    session_id int references sessions(session_id),
    time_offset smallint,
    weather weather_condition,
    track_temp smallint,
    track_temp_change smallint,
    air_temp smallint,
    air_temp_change smallint,
    rain_density smallint
);

create table if not exists marshal_zones (
    marshal_zone_id serial primary key,
    session_id int references sessions(session_id),
    zone_start float not null,
    zone_flag int not null
);

create table if not exists lap_data (
    lap_id serial primary key,
    session_id int references sessions(session_id),
    last_lap_time int,
    current_lap_time int,
    sector1_time_in_ms int,
    sector1_time_in_s smallint,
    sector2_time_in_ms int,
    sector2_time_in_s smallint,
    sector3_time_in_ms int,
    sector3_time_in_s smallint,
    delta_to_car_ahead int,
    delta_to_leader int,
    lap_distance float,
    total_distance float,
    safety_car_delta float,
    car_position smallint,
    current_lap_num smallint,
    pit_status smallint,
    num_pit_stops smallint,
    sector smallint,
    current_lap_invalid smallint,
    penalties smallint,
    total_wanings smallint,
    corner_cutting_warnings smallint,
    num_unserved_drive_through_pens smallint,
    num_unserved_stop_go_pens smallint,
    grid_position smallint,
    driver_status smallint,
    result_status smallint,
    pit_lane_timer_active smallint,
    pit_lane_time_in_lane int,
    pit_stop_timer_in_ms int,
    pit_stop_should_serve_penalty smallint
);

create table if not exists car_setup_data (
    setup_id serial primary key,
    session_id int references sessions(session_id),
    front_wing smallint,
    rear_wing smallint,
    on_throttle smallint,
    off_throttle smallint,
    front_camber smallint,
    rear_camber smallint,
    front_toe smallint,
    rear_toe smallint,
    front_suspension smallint,
    rear_suspension smallint,
    front_anti_roll_bar smallint,
    rear_anti_roll_bar smallint,
    front_suspension_height smallint,
    rear_suspension_height smallint,
    brake_pressure smallint,
    brake_bias smallint,
    rear_left_tyre_pressure float,
    rear_right_tyre_pressure float,
    front_left_tyre_pressure float,
    front_right_tyre_pressure float,
    ballast smallint,
    fuel_load float
);

create table if not exists car_telemetry (
    telemetry_id serial primary key,
    session_id int references sessions(session_id),
    mfd_panel smallint,
    suggested_gear smallint,
    speed smallint,
    throttle float,
    steer float,
    brake float,
    clutch smallint,
    gear smallint,
    engine_rpm int,
    drs smallint,
    rev_lights_percent smallint,
    rl_brake_temp_celsius smallint,
    rr_brake_temp_celsius smallint,
    fl_brake_temp_celsius smallint,
    fr_brake_temp_celsius smallint,
    rl_tyre_surface_temp smallint,
    rr_tyre_surface_temp smallint,
    fl_tyre_surface_temp smallint,
    fr_tyre_surface_temp smallint,
    rl_tyre_inner_temp smallint,
    rr_tyre_inner_temp smallint,
    fl_tyre_inner_temp smallint,
    fr_tyre_inner_temp smallint,
    engine_temp int,
    rl_tyre_pressure float,
    rr_tyre_pressure float,
    fl_tyre_pressure float,
    fr_tyre_pressure float,
    rl_surface_type smallint,
    rr_surface_type smallint,
    fl_surface_type smallint,
    fr_surface_type smallint
);

create table if not exists car_status_static (
    status_id serial primary key,
    session_id int references sessions(session_id),
    traction_control smallint,
    anti_lock_brakes smallint,
    fuel_capacity float,
    max_rpm int,
    idle_rpm int,
    max_gears smallint
);

create table if not exists car_status_dynamic (
    dynamic_status_id serial primary key,
    session_id int references sessions(session_id),
    status_id int references car_status_static(status_id),
    fuel_mix smallint,
    front_brake_bias smallint,
    pit_limiter_status smallint,
    fuel_in_tank float,
    fuel_remaining_laps float,
    drs_allowed smallint,
    drs_activation_distance int,
    actual_tyre_compound tyre_compound,
    visual_tyre_compound tyre_compound,
    tyres_age_laps smallint,
    vehicle_fia_flags smallint,
    engine_power_ice float,
    engine_power_mguk float,
    ers_energy_store float,
    ers_deploy_mode ers_mode,
    ers_harvested_this_lap_mguk float,
    ers_harvested_this_lap_mguh float,
    ers_deployed_this_lap float
);

create table if not exists final_classification (
    classification_id serial primary key,
    session_id int references sessions(session_id),
    finish_position smallint,
    num_laps smallint,
    grid_position smallint,
    points smallint,
    num_pit_stops smallint,
    result_status smallint,
    best_lap_time int,
    total_race_time float,
    penalties smallint,
    num_penalties smallint,
    num_tyre_stints smallint
    -- There is some stuff for tyre stints here to maybe add later
);

create table if not exists car_damage (
    damage_id serial primary key,
    session_id int references sessions(session_id),
    tyre_wear_rl float,
    tyre_wear_rr float,
    tyre_wear_fl float,
    tyre_wear_fr float,
    tyre_damage_rl smallint,
    tyre_damage_rr smallint,
    tyre_damage_fl smallint,
    tyre_damage_fr smallint,
    front_left_wing_damage smallint,
    front_right_wing_damage smallint,
    rear_wing_damage smallint,
    floor_damage smallint,
    diffuser_damage smallint,
    sidepod_damage smallint,
    drs_fault smallint,
    ers_fault smallint,
    gear_box_damage smallint,
    engine_damage smallint,
    engine_mguh_wear smallint,
    engine_es_wear smallint,
    engine_ce_wear smallint,
    engine_mgu_k_wear smallint,
    engine_wear_tc smallint,
    engine_blown smallint,
    engine_seized smallint
);

create table if not exists motion_ex (
    motion_ex_id serial primary key,
    session_id int references sessions(session_id),
    rl_suspension_position float,
    rr_suspension_position float,
    fl_suspension_position float,
    fr_suspension_position float,
    rl_suspension_velocity float,
    rr_suspension_velocity float,
    fl_suspension_velocity float,
    fr_suspension_velocity float,
    rl_suspension_acceleration float,
    rr_suspension_acceleration float,
    fl_suspension_acceleration float,
    fr_suspension_acceleration float,
    rl_wheel_speed float,
    rr_wheel_speed float,
    fl_wheel_speed float,
    fr_wheel_speed float,
    rl_slip_ratio float,
    rr_slip_ratio float,
    fl_slip_ratio float,
    fr_slip_ratio float,
    rl_slip_angle float,
    rr_slip_angle float,
    fl_slip_angle float,
    fr_slip_angle float,
    rl_wheel_lat_force float,
    rr_wheel_lat_force float,
    fl_wheel_lat_force float,
    fr_wheel_lat_force float,
    rl_wheel_lon_force float,
    rr_wheel_lon_force float,
    fl_wheel_lon_force float,
    fr_wheel_lon_force float,
    height_cog float,
    local_velocity_x float,
    local_velocity_y float,
    local_velocity_z float,
    angular_velocity_x float,
    angular_velocity_y float,
    angular_velocity_z float,
    angular_acceleration_x float,
    angular_acceleration_y float,
    angular_acceleration_z float,
    front_wheels_angle float,
    rl_wheel_vertical_load float,
    rr_wheel_vertical_load float,
    fl_wheel_vertical_load float,
    fr_wheel_vertical_load float
);

-- TODO: Finish defining tables for event data
-- Event Data
create table if not exists event_data (
    event_id serial primary key,
    session_id int references sessions(session_id),
    event_time int,
    event_type event_type
);

create table if not exists overtake_data (
    overtake_id serial primary key,
    session_id int references event_data(event_id),
    overtaking_car_id int,
    overtaken_car_id int
);
-- !TODO
