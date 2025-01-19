use crate::maths_rs::abs;
use crate::maths_rs::signum;
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
    mode: RadarMode,
    azimuth_inc: f64,
    azimuth: f64,
    beam_width: f64,
    contact: Vec<Contact>,
}

pub struct Contact {
    class: Class,
    position: Vec2,
    velocity: Vec2,
    acc_to_intercept: Vec2,
    trq_to_intercept: f64,
    suggest_fire_cmd: bool,
    engaged: bool,
}

pub struct Ship {
    radar: Radar,
}

impl Ship {
    pub fn new() -> Ship {
        Ship {
            radar: Radar {
                mode: RadarMode::RWS,
                azimuth_inc: 0.0,
                azimuth: 0.0,
                beam_width: 0.0,
                //contact: Contact {
                //    class: Class::Unknown,
                //},
                contact: vec![],
            },
        }
    }

    pub fn update_radar(&mut self) {
        if self.radar.mode == RadarMode::RWS {
            self.radar.azimuth_inc = 0.65;
            self.radar.beam_width = 2.5;
        } else if self.radar.mode == RadarMode::STT {
            self.radar.azimuth_inc = 0.0;
            self.radar.beam_width = 0.2;
        }

        self.radar.azimuth += self.radar.azimuth_inc;

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

            if self.radar.contact.is_empty() {
                self.radar.contact.push(n_contact);
            } else {
                self.radar.contact[0] = n_contact;
            }
            self.radar.mode = RadarMode::STT;
            self.radar.azimuth = (self.radar.contact[0].position - position()).angle();
            let dp = contact.position - position();
            let t_start = dp.length() / BULLET_SPEED;
            //let a = contact.velocity - self.velocity;
            let a = 0.0;
            let posi_to_shot =
                contact.position + contact.velocity * t_start + 0.5 * a * t_start * t_start;
            let dp2 = posi_to_shot - position();
            let t = dp2.length() / BULLET_SPEED;
            let posi_to_shot2 = contact.position + target_velocity() * t + 0.5 * a * t * t;
            let ang_diff = angle_diff(heading(), (posi_to_shot2 - position()).angle());
            //draw_line(position(), posi_to_shot2, 0xffff00);

            let safety_factor = 1.03;
            let ang_breaking_distance =
                0.5 * angular_velocity() * angular_velocity() / FIGHTER_MAX_TORQUE * safety_factor;

            let distance = (dp2[0] * dp2[0] + dp2[1] * dp2[1]).sqrt();
            let v_diff = contact.velocity - velocity();
            debug!("v_diff: {}", v_diff);
            //let position_angle = (dp2[1] / dp2[0]).atan();
            let v = (v_diff[0] * v_diff[0] + v_diff[1] * v_diff[1]).sqrt();
            let v_rad = ang_diff.cos() * v;
            debug!("v_rad: {}", v_rad);

            let offset = 100.0; // meter
            let breaking_distance =
                0.5 * v_rad * v_rad / FIGHTER_MAX_TORQUE * safety_factor + offset;
            let k3 = signum(distance - breaking_distance);
            let k4;

            if v_rad < 0.0 {
                k4 = 1.0;
            } else {
                k4 = k3;
            }

            self.radar.contact[0].acc_to_intercept =
                vec2(heading().cos(), heading().sin()).normalize() * FIGHTER_MAX_ACC * k4;

            //let rel_vel = contact.velocity - velocity();

            debug!(
                "acc_to_intercept: {}",
                v_diff.normalize() * FIGHTER_MAX_ACC * k4
            );
            debug!("k4: {}", k4);
            debug!("breaking_distance: {}", breaking_distance);
            debug!("distance: {}", distance);

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

            self.radar.contact[0].trq_to_intercept = FIGHTER_MAX_TORQUE * s4;
            if abs(ang_diff) < 0.03 {
                self.radar.contact[0].suggest_fire_cmd = true;
            } else {
                self.radar.contact[0].suggest_fire_cmd = false;
            }
        } else {
            self.radar.mode = RadarMode::RWS;
        }

        set_radar_heading(self.radar.azimuth);
        set_radar_width(self.radar.beam_width);
    }

    pub fn tick(&mut self) {
        self.update_radar();

        if !self.radar.contact.is_empty() {
            torque(self.radar.contact[0].trq_to_intercept);
            accelerate(self.radar.contact[0].acc_to_intercept);
            if self.radar.contact[0].suggest_fire_cmd {
                fire(0);
            }
        }
    }
}
