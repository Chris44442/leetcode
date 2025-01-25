use oort_api::prelude::*;

#[derive(Default, PartialEq, Eq)]
pub enum RadarMode {
    #[default]
    RWS,
    STT,
    TWS,
}

pub struct Radar {
    pub mode: RadarMode,
    pub azimuth_inc: f64,
    pub azimuth: f64,
    pub beam_width: f64,
    pub contact: Vec<Contact>,
}

pub struct Contact {
    pub class: Class,
    pub position: Vec2,
    pub velocity: Vec2,
    pub acc_to_intercept: Vec2,
    pub trq_to_intercept: f64,
    pub suggest_fire_cmd: bool,
    pub engaged: bool,
}
