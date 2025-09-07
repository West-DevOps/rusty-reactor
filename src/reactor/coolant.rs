use crate::{units, constants};

#[derive(Debug)]
pub struct Loop {
    inlet_temperature: units::Kelvin,
    outlet_temperature: units::Kelvin,
}

impl Loop {
    pub fn new() -> Loop {
        Loop {
            inlet_temperature: constants::ROOM_TEMPERATURE,
            outlet_temperature: constants::ROOM_TEMPERATURE,
        }
    }

    fn spin_pump(self, temperature: units::Kelvin, exchanger: Exchanger) -> f64 {
        temperature / 100.0f64 * exchanger.get_efficiency() as f64
    }
}

#[derive(Debug)]
pub struct Exchanger {
    efficiency: u8,
}

impl Exchanger {
    pub fn new(efficiency: u8) -> Exchanger {
        Exchanger {
            efficiency: efficiency,
        }
    }

    pub fn get_efficiency(&self) -> u8 {
        self.efficiency
    }

    fn exchange(&self, input_temperature: f32) -> f32 {
        input_temperature * self.efficiency as f32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn loop_can_be_pumped() {
        let loop_1: Loop = Loop::new();
        let ex = Exchanger::new(50);

        assert_eq!(250.0, loop_1.spin_pump(500.0, ex));
    }

    #[test]
    fn exchanger_get_efficiency() {
        let exchg = Exchanger::new(75);
        assert_eq!(75, exchg.get_efficiency());
    }
}
