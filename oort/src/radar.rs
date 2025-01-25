use crate::maths_rs::abs;
use crate::maths_rs::signum;
use maths_rs::distance;
use oort_api::prelude::*;
const BULLET_SPEED: f64 = 1000.0; // m/s
const FIGHTER_MAX_TORQUE: f64 = 2.0 * PI; // m/s^2
const FIGHTER_MAX_ACC: f64 = 60.0; // m/s^2

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
    pub last_velocity: Vec2,
}

impl Radar {
    pub fn new() -> Self {
        Radar {
            mode: RadarMode::RWS,
            azimuth_inc: 0.0,
            azimuth: 0.0,
            beam_width: 0.0,
            contact: vec![],
            last_velocity: vec2(0.0, 0.0),
        }
    }

    pub fn update(&mut self) {
        if self.mode == RadarMode::RWS {
            self.azimuth_inc = 0.65;
            self.beam_width = 2.5;
        } else if self.mode == RadarMode::STT {
            self.azimuth_inc = 0.0;
            self.beam_width = 0.2;
        }

        self.azimuth += self.azimuth_inc;

        if let Some(contact) = scan() {
            let n_contact: Contact = Contact {
                class: contact.class,
                position: contact.position,
                velocity: contact.velocity,
                acc_to_intercept: vec2(0.0, 0.0),
                trq_to_intercept: 0.0,
                suggest_fire_cmd: false,
                engaged: true,
            };

            if self.contact.is_empty() {
                self.contact.push(n_contact);
            } else {
                self.contact[0] = n_contact;
            }
            self.mode = RadarMode::STT;
            self.azimuth = (self.contact[0].position - position()).angle();
            let dp = contact.position - position();
            let dv1 = contact.velocity - velocity();
            let abs_bullet_spd_x = BULLET_SPEED * heading().cos() + velocity()[0];
            let abs_bullet_spd_y = BULLET_SPEED * heading().sin() + velocity()[1];
            debug!("abs_bullet_spd_x = {}", abs_bullet_spd_x);
            debug!("abs_bullet_spd_y = {}", abs_bullet_spd_y);

            let abs_blt_spd =
                (abs_bullet_spd_x * abs_bullet_spd_x + abs_bullet_spd_y * abs_bullet_spd_y).sqrt();

            //let t_start = dp.length() / BULLET_SPEED;
            let t_start = dp.length() / abs_blt_spd;
            debug!("abs_bullet_spd = {}", abs_blt_spd);
            let a = velocity() - self.last_velocity;
            self.last_velocity = velocity();
            //let a = 0.0;
            let posi_to_shot =
                contact.position + contact.velocity * t_start + 0.5 * a * t_start * t_start;
            let dp2 = posi_to_shot - position();
            let t = dp2.length() / abs_blt_spd;
            let posi_to_shot2 = contact.position + dv1 * t + 0.5 * a * t * t;
            let ang_diff = angle_diff(heading(), (posi_to_shot2 - position()).angle());
            //draw_line(position(), posi_to_shot2, 0xffff00);

            let safety_factor = 1.03;
            let ang_breaking_distance =
                0.5 * angular_velocity() * angular_velocity() / FIGHTER_MAX_TORQUE * safety_factor;

            let s1 = signum(ang_diff);
            let s2 = signum(angular_velocity());
            let s3 = signum(abs(ang_diff) - ang_breaking_distance);
            let s4;

            if s1 != s2 {
                // as long as our ang_velocity goes in the wrong direction, always burn the other way
                s4 = s1;
            } else {
                // if our ang_velocity goes the right way, we need to consider our breaking distance
                s4 = s1 * s3;
            }

            self.contact[0].trq_to_intercept = FIGHTER_MAX_TORQUE * s4;

            if abs(ang_diff) < 0.03 {
                let distance = (dp2[0] * dp2[0] + dp2[1] * dp2[1]).sqrt();
                debug!("distance = {}", distance);
                if distance < 10550.0 {
                    self.contact[0].suggest_fire_cmd = true;
                }
            } else {
                self.contact[0].suggest_fire_cmd = false;
            }
        } else {
            self.mode = RadarMode::RWS;
        }

        set_radar_heading(self.azimuth);
        set_radar_width(self.beam_width);
    }
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

impl Contact {
    pub fn new(class: Class) -> Self {
        Contact {
            class,
            position: vec2(0.0, 0.0),
            velocity: vec2(0.0, 0.0),
            acc_to_intercept: vec2(0.0, 0.0),
            trq_to_intercept: 0.0,
            suggest_fire_cmd: false,
            engaged: false,
        }
    }
}
