#[derive(Debug)]
pub struct WheelData<T> {
    pub rear_left: T,
    pub rear_right: T,
    pub front_left: T,
    pub front_right: T,
}

#[derive(Debug)]
pub enum SurfaceType {
    Tarmac,
    RumbleStrip,
    Concrete,
    Rock,
    Gravel,
    Mud,
    Sand,
    Grass,
    Water,
    Cobblestone,
    Metal,
    Ridged,
}

#[derive(Debug, Default)]
pub enum TyreCompound {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    Inter,
    Wet,
    ClassicDry,
    ClassicWet,
    F2SuperSoft,
    F2Soft,
    F2Medium,
    F2Hard,
    F2Wet,
    #[default]
    Invalid,
}
