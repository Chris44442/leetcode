use oort_api::prelude::*;

pub struct Radio {
    pub last_velocity: Vec2,
}

impl Radio {
    pub fn new() -> Self {
        Radio {
            last_velocity: vec2(0.0, 0.0),
        }
    }

    pub fn update(&mut self) {
        set_radio_channel(2);
        let contact_position: Vec2;
        let contact_velocity: Vec2;
        if let Some(msg) = receive() {
            debug!("msg: {msg:?}");
            contact_position = vec2(msg[0], msg[1]);
            contact_velocity = vec2(msg[2], msg[3]);
        } else {
            debug!("no message received");
        }
    }
}
