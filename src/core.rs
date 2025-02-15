use rand::random_bool;

pub const ROOM_TEMPERATURE: f32 = 293.15;
pub const U_MELT_PT: f32 = 1405.3;
// pub const U_BOIL_PT: f32 = 4404.0;

const BASE_HEAT: f32 = 0.143;
const BASE_MASS_LOSS: f32 = 0.012;

pub struct Core {
    u_mass: f32,
    temperature: f32,
    control_rod_postion: u8,
}

impl Core {
    pub fn scram(&mut self) -> Result<String, String> {
        if self.temperature >= U_MELT_PT {
            return Err(String::from("MELTDOWN!")); // Not much we can do now!
        } 

        self.control_rod_postion = 100; // Drop the rods
        Ok(String::from("SCRAMMED REACTOR!"))
    }

    pub fn decay(&mut self) -> Result<bool, String> {
        if self.u_mass <= 0.0 {
            return Err(String::from("OUT OF FUEL!"));
        }

        // use random to work out if something decayed, 
        // more likely if control rods are more withdrawn (closer to 0).
        if !random_bool(self.control_rod_postion as f64 / 99.9) {
            self.u_mass -= BASE_MASS_LOSS;
            self.temperature += BASE_HEAT;

            if self.temperature >= 1405.3 {
                return Err(String::from("MELTDOWN!"));
            }

        } else {
            // Nothing decayed, lose some heat
            if self.temperature > ROOM_TEMPERATURE {
                self.temperature -= BASE_HEAT;
            }
            return Ok(false);
        }
        Ok(true)
    }

    pub fn get_u_mass(&self) -> f32 {
        self.u_mass
    }

    pub fn get_temperature(&self) -> f32 {
        self.temperature
    }

    pub fn get_rod_position(&self) -> u8 {
        self.control_rod_postion
    }

    pub fn withdraw_rods(&mut self, amount: u8) -> Result<u8, String> {
        let new_position = self.control_rod_postion - amount;
        if new_position <= 0 {
            return Err(String::from("Cannot pull rods by that amount"));
        } else {
            self.control_rod_postion -= amount;
            return Ok(self.control_rod_postion);
        }
    }

    pub fn insert_rods(&mut self, amount: u8) -> Result<u8, String> {
        if self.control_rod_postion + amount >= 100 {
            return Err(String::from("Cannot insert rods that much"));
        } else {
            self.control_rod_postion += amount;
            return Ok(self.control_rod_postion);
        }
    }

}

pub fn build_core(fuel_load: f32) -> Core {
    Core {
        u_mass: fuel_load, // Grams of Uranium in core
        temperature: ROOM_TEMPERATURE, // Kelvin
        control_rod_postion: 100, // 100% (fully in the core, totally choking the reaction)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn starts_at_ambient_temp() {
        let c = build_core(50000.5);
        assert_eq!(c.get_temperature(), ROOM_TEMPERATURE);
    }
}
