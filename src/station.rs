use log::info;

use crate::reactor::Reactor;
use crate::turbine::TurbineHall;
use crate::control::ControlRoom;

/// Struct to encapsulate all of the individual components of thew reactor. 
pub struct Station {
    pub reactor: Reactor,
    pub turbine_hall: TurbineHall,
    pub control_room: ControlRoom,
}

impl Station {
    /// Creates a new instance of the Station
    pub fn new(fuel_load: f32, exchanger_efficiency: u8, ratio: u8) -> Station {
        Station {
            reactor: Reactor::new(fuel_load, exchanger_efficiency),
            turbine_hall: TurbineHall::new(ratio),
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
        let st: Station = Station::new(50000000.0, 76, 2);
        st.run();
    }
}
