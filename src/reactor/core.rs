use rand::random_bool;
use crate::{units, constants};

const BASE_HEAT: f64 = 0.143;
const BASE_MASS_LOSS: f64 = 0.012;

#[derive(Debug)]
pub struct Core {
    u_mass: units::Gram,
    temperature: units::Kelvin,
    control_rod_postion: units::RodPosition,
}

impl Core {
    pub fn new(fuel_load: units::Gram) -> Core { 
        Core {
            u_mass: fuel_load, // Grams of Uranium in core (determines how long the core will last)
            temperature: constants::ROOM_TEMPERATURE, // Kelvin, this gets > U_MELT_PT = GAME OVER!
            control_rod_postion: 100, // 100% (fully inserted into the core, totally choking the reaction)
        }
    }

    pub fn scram(&mut self) -> Result<&str, &str> {
        if self.temperature >= constants::U_PROPERTIES.melting_point {
            return Err("MELTDOWN!"); // Not much we can do now!
        } 

        self.control_rod_postion = 100; // Drop the rods
        Ok("SCRAMMED REACTOR!")
    }

    pub fn decay(&mut self) -> Result<bool, &str> {
        if self.u_mass <= 0.0 {
            return Err("OUT OF FUEL!");
        }

        // use random to work out if something decayed, 
        // more likely if control rods are more withdrawn (closer to 0).
        if !random_bool(self.control_rod_postion as f64 / 99.9) {
            self.u_mass -= BASE_MASS_LOSS;
            self.temperature += BASE_HEAT;

            if self.temperature >= 1405.3 {
                return Err("MELTDOWN!");
            }

        } else {
            // Nothing decayed, lose some heat
            if self.temperature > constants::ROOM_TEMPERATURE {
                self.temperature -= BASE_HEAT;
            }
            return Ok(false);
        }
        Ok(true)
    }

    pub fn get_u_mass(&self) -> units::Gram {
        self.u_mass
    }

    pub fn get_temperature(&self) -> units::Kelvin {
        self.temperature
    }

    pub fn get_rod_position(&self) -> units::RodPosition {
        self.control_rod_postion
    }

    pub fn set_rod_position(&mut self, value: units::RodPosition) -> Result<units::RodPosition, &str> {
        if value > 100 {
            return Err("Rod position must be 0-100");
        }
        self.control_rod_postion = value;
        Ok(self.control_rod_postion)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn starts_at_ambient_temp() {
        let c = Core::new(50000.5);
        assert_eq!(c.get_temperature(), constants::ROOM_TEMPERATURE);
    }

    #[test]
    fn reactor_rods_are_fully_inserted_on_start() {
        let c: Core = Core::new(50000.5);
        assert_eq!(c.control_rod_postion, 100);
    }
}
