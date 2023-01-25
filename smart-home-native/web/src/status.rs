use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Status {
    pub desired_temp: f32,
    pub actual_temp: f32,
    pub auto_heating: bool,
    pub heating_on: bool,
    pub switch_on: bool
}

impl Status {
    pub fn new() -> Status {
        Status {
            desired_temp: 0.0,
            actual_temp: 0.0,
            auto_heating: false,
            heating_on: false,
            switch_on: false
        }
    }
}
