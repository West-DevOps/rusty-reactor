mod core;
pub mod coolant;

use core::Core;
use coolant::{ Loop, Exchanger };

pub struct Reactor {
    core: Core,
    primary_loop: Loop,
    secondary_loop: Loop,
    heat_exchanger: Exchanger
}

impl Reactor {
    pub fn new(fuel_load: f32, exhanger_efficency: u8) -> Reactor {
        Reactor {
            core: Core::new(fuel_load),
            primary_loop: Loop::new(),
            secondary_loop: Loop::new(),
            heat_exchanger: Exchanger::new(exhanger_efficency),
        }
    }

    pub fn get_secondary_loop(self) -> Loop {
        self.secondary_loop
    }
}
