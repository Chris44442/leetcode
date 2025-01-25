use oort_api::prelude::*;

mod propulsion;
use propulsion::*;
mod radar;
use radar::*;

pub struct Ship {
    radar: Radar,
    prop: Propulsion,
}

impl Ship {
    pub fn new() -> Ship {
        Ship {
            radar: Radar::new(),
            prop: Propulsion::new(),
        }
    }

    pub fn tick(&mut self) {
        self.radar.update();

        let mut posi = vec2(0.0, 0.0);
        //if current_tick() < 1000 {
        //    posi = vec2(50.0, 50.0);
        //} else {
        //    posi = position();
        //}

        //debug!("{}", current_tick());

        if !self.radar.contact.is_empty() {
            torque(self.radar.contact[0].trq_to_intercept);
            //accelerate(self.radar.contact[0].acc_to_intercept);
            posi = self.radar.contact[0].position;
            if self.radar.contact[0].suggest_fire_cmd {
                fire(0);
            }
        } else {
            posi = position();
        }
        self.prop.update(posi);
    }
}
