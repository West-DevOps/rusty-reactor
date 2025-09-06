mod core;
pub mod coolant;

use core::Core;
use coolant::{ Loop, Exchanger };

/// For grouping together the resources of the reactor, uses `start()` function to start the main `Core` `loop {}`.
pub struct Reactor {
    core: Core,
    primary_loop: Loop,
    secondary_loop: Loop,
    heat_exchanger: Exchanger
}

impl Reactor {
    /// Create a new instance of the Reactor object / thread.
    pub fn new(fuel_load: f64, exchanger_efficiency: u8) -> Reactor {
        Reactor {
            core: Core::new(fuel_load),
            primary_loop: Loop::new(),
            secondary_loop: Loop::new(),
            heat_exchanger: Exchanger::new(exchanger_efficiency),
        }
    }

    pub fn get_secondary_loop(self) -> Loop {
        self.secondary_loop
    }

    pub fn start(self) {
        loop {
            
        }
    }
}
