mod core;
pub mod coolant;

use crate::units;
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
    pub fn new(fuel_load: units::Gram, exchanger_efficiency: units::Percent) -> Reactor {
        Reactor {
            core: Core::new(fuel_load),
            primary_loop: Loop::new(),
            secondary_loop: Loop::new(),
            heat_exchanger: Exchanger::new(exchanger_efficiency),
        }
    }

    /// Get the secondary loop object. 
    pub fn get_secondary_loop(self) -> Loop {
        self.secondary_loop
    }

    // Main reactor thread code.  Started by `main.rs`
    pub fn start(self) {
        loop {
            
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reactor_new() {
        let mut rct = Reactor::new(5000f64, 50u8);
        rct.get_secondary_loop();
    }
}