use crate::reactor::Reactor;
use crate::turbine::TurbineHall;
use crate::control::ControlRoom;

pub struct Station {
    reactor: Reactor,
    turbine_hall: TurbineHall,
    control_room: ControlRoom,
}

impl Station {
    pub fn new(fuel_load: f32, exchanger_efficiency: u8, ratio: u8) -> Station {
        Station {
            reactor: Reactor::new(fuel_load, exchanger_efficiency),
            turbine_hall: TurbineHall::new(ratio),
            control_room: ControlRoom::new(),
        }
    }

    pub fn run(&mut self) {
        println!("entered run of station");
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn station_create() {
        let mut st: Station = Station::new(50000000.0, 76, 2);
        st.run();
    }
}
