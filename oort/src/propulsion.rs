use oort_api::prelude::*;

pub struct Propulsion {
    pd_x: PID,
    pd_y: PID,
}

impl Propulsion {
    pub fn new() -> Self {
        Propulsion {
            pd_x: PID::new(0.2, 85.0, 0.0),
            pd_y: PID::new(0.2, 85.0, 0.0),
        }
    }
    pub fn update(&mut self, target_posi: Vec2) {
        let posi = position();
        self.pd_x.setpoint = target_posi[0];
        self.pd_y.setpoint = target_posi[1];
        let x_a = self.pd_x.update(posi[0]);
        let y_a = self.pd_y.update(posi[1]);
        let accel = vec2(x_a, y_a);
        accelerate(accel);
    }
}

pub struct PID {
    kp: f64,
    kd: f64,
    setpoint: f64,
    prev_error: f64,
}

impl PID {
    pub fn new(kp: f64, kd: f64, setpoint: f64) -> Self {
        PID {
            kp,
            kd,
            setpoint,
            prev_error: 0.0,
        }
    }

    pub fn update(&mut self, input: f64) -> f64 {
        let error = self.setpoint - input;
        //debug!("pid error = {}", error);
        let derivative = error - self.prev_error;
        //debug!("pid derivative = {}", derivative);
        let output = self.kp * error + self.kd * derivative;
        //debug!("pid output = {}", output);
        self.prev_error = error;
        output
    }
}

//mod moves;
//use moves::PID;
//pub struct Ship {
//}
//
//impl Ship {
//    pub fn new() -> Ship {
//        Ship {
//            pd_x: PID::new(0.2, 85.0, 0.0),
//            pd_y: PID::new(0.2, 85.0, 0.0),
//        }
//    }
//
//    pub fn tick(&mut self) {
//        let posi = position();
//        self.pd_x.setpoint = 300.0;
//        self.pd_y.setpoint = -500.0;
//
//        let x_a = self.pd_x.compute(posi[0]);
//        let y_a = self.pd_y.compute(posi[1]);
//        //let y_a = 0.0;
//
//        let accel = vec2(x_a, y_a);
//
//        accelerate(accel);
