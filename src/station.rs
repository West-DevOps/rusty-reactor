use log::info;

use crate::reactor::Reactor;
use crate::control::ControlRoom;

/// Struct to encapsulate all of the individual components of thew reactor. 
pub struct Station {
    pub reactor: Reactor,
    pub control_room: ControlRoom,
}

impl Station {
    /// Creates a new instance of the Station
    pub fn new(fuel_load: f64, exchanger_efficiency: u8) -> Station {
        Station {
            reactor: Reactor::new(fuel_load, exchanger_efficiency),
            control_room: ControlRoom::new(),
        }
    }

    pub fn run(&self) {
        info!("entered run of station");
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn station_create() {
        let st: Station = Station::new(50000000.0f64, 76);
        st.run();
    }
}
