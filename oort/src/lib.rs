use crate::maths_rs::abs;
use crate::maths_rs::signum;
use oort_api::prelude::*;

const BULLET_SPEED: f64 = 1000.0; // m/s
const FIGHTER_MAX_TORQUE: f64 = 2.0 * PI; // m/s^2

pub struct Ship {
    sweeping: bool,
    next_radar_heading: f64,
    velocity: f64,
}

impl Ship {
    pub fn new() -> Ship {
        Ship {
            sweeping: true,
            next_radar_heading: 0.0,
            velocity: 0.0,
        }
    }

    pub fn tick(&mut self) {
        let mut azimuth_inc: f64 = 0.0;
        if self.sweeping {
            azimuth_inc = 0.03;
        }

        //set_radar_heading(radar_heading() + azimuth_inc);
        set_radar_heading(self.next_radar_heading + azimuth_inc);
        set_radar_width(0.2);
        if let Some(contact) = scan() {
            self.sweeping = false;
            self.next_radar_heading =
                angle_diff(heading(), contact.position.angle() - position().angle());
            //accelerate(0.1 * (contact.position - position()));
            //fire(0);
            //turn(f64::signum(angle_diff(heading(), (target() - position()).angle())));

            self.next_radar_heading = (contact.position - position()).angle();

            //draw_line(position(), contact.position, 0x00ff00);
            let dp = contact.position - position();
            let t_start = dp.length() / BULLET_SPEED;
            let a = contact.velocity - self.velocity;
            let a = 0.0;
            let posi_to_shot =
                contact.position + contact.velocity * t_start + 0.5 * a * t_start * t_start;
            let dp2 = posi_to_shot - position();
            let t = dp2.length() / BULLET_SPEED;
            let posi_to_shot2 = contact.position + target_velocity() * t + 0.5 * a * t * t;
            let ang_diff = angle_diff(heading(), (posi_to_shot2 - position()).angle());
            //draw_line(position(), posi_to_shot2, 0xffff00);

            let safety_factor = 1.03;
            let breaking_distance =
                0.5 * angular_velocity() * angular_velocity() / FIGHTER_MAX_TORQUE * safety_factor;

            debug!("ang_diff: {}", ang_diff);
            debug!("ang_velocity: {}", angular_velocity());
            debug!("ds: {}", breaking_distance);
            let s1 = signum(ang_diff);
            let s2 = signum(angular_velocity());
            let s3 = signum(abs(ang_diff) - breaking_distance);
            let s4;

            if s1 != s2 {
                // as long as our ang_velocity goes in the wrong direction, always burn the other way
                s4 = s1;
            } else {
                // if our ang_velocity goes the right way, we need to consider our breaking distance
                s4 = s1 * s3;
            }

            if abs(ang_diff) > 0.000 {
                let trq = FIGHTER_MAX_TORQUE * s4;
                debug!("torque: {}", trq);
                torque(trq);
                //if ang_diff > 0.0 {
                //    debug!("pos diff angle");
                //    if ds < f64::abs(ang_diff) {
                //        torque(FIGHTER_MAX_TORQUE);
                //    } else {
                //        torque(-FIGHTER_MAX_TORQUE);
                //    }
                //} else {
                //    debug!("neg diff angle");
                //    if ds < f64::abs(ang_diff) {
                //        torque(-FIGHTER_MAX_TORQUE);
                //    } else {
                //        torque(FIGHTER_MAX_TORQUE);
                //    }
                //}
            }
            if abs(ang_diff) < 0.03 {
                fire(0);
            }
        } else {
            self.sweeping = true;
            self.next_radar_heading = radar_heading();
        }
    }
}
