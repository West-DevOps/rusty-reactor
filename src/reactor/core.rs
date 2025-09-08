
use rand::random_bool;
use crate::{units, constants};

const BASE_HEAT: f64 = 0.143;
const BASE_MASS_LOSS: f64 = 0.012;


#[derive(Copy, Clone, Debug)]
/// The lowest level of the Core, the fuel elements own the decay probability and heat generation code
struct FuelElement {
    temperature: units::Kelvin,
    fuel_mass: units::Gram,
}

impl FuelElement {
    /// Create a new fuel element, at room temperature with a given mass of fuel
    fn new(fuel_mass: units::Gram) -> FuelElement {
        FuelElement { temperature: constants::ROOM_TEMPERATURE, fuel_mass }
    }


    /// Main decay function of the code, reduces the amount of fuel and increases heat of the element.
    fn decay(&mut self, rod_position: units::RodPosition) -> Result<bool, String> {
        if self.temperature > constants::U_PROPERTIES.melting_point {
            return Err(format!("FuelElement has melted with rod position at {}", rod_position));
        }
        
        if random_bool(rod_position as f64 / 100f64) {
            // Here we need to model how much energy is emitted from the fission event
            // Then, using the mass and heat capacity of the fuel we can model the  increase in temperature for a given amount of energy created
            self.fuel_mass -= BASE_MASS_LOSS;
            self.temperature += BASE_HEAT;
            Ok(true)
        } else {
            // No decay happened for this element this loop, just reduce heat by arbitrary amount for now
            // Future will see how much it should cool by reading the state of the coolant in the channel next to it
            self.temperature -= 0.082;
            Ok(false)
        }
    }
}

#[derive(Debug)]
/// Core struct, maintains the FuelElement array, average core temp and a few other critical core components
pub(super) struct Core {
    /// A grid of fuel elements
    fuel_elements: [[FuelElement; 255]; 255],

    /// 0% means the rods are 0% _withdrawn_ (i.e. fully inserted into the core, choking the reaction).  100% is full-gas (fully withdrawn)
    control_rod_postion: units::RodPosition,

    /// Do we want to panic!() the program if things get too hot?
    panic_on_meltdown: bool,
}

impl Core {
    pub fn new(fuel_mass_per_element: units::Gram, panic_on_meltdown: bool) -> Core { 
        Core {
            fuel_elements: [[FuelElement::new(fuel_mass_per_element); 255]; 255],
            control_rod_postion: 0,
            panic_on_meltdown,
        }
    }

    pub fn scram(&mut self) -> Result<&str, &str> {
        if self.get_temperature() >= constants::U_PROPERTIES.melting_point {
            return Err("MELTDOWN!"); // Not much we can do now!
        } 

        self.control_rod_postion = 0; // Drop the rods
        Ok("SCRAMMED REACTOR!")
    }

    pub(super) fn decay(&mut self) -> Result<(), String> {
        if self.get_temperature() > constants::U_PROPERTIES.melting_point {
            return Err(String::from("Core Meltdown"));
        }
        
        for array in self.fuel_elements.iter_mut() {
            for element in array.iter_mut() {
                match element.decay(self.control_rod_postion) {
                    Ok(_) => {}
                    Err(s) => { 
                        if self.panic_on_meltdown {
                            panic!("{}", s);
                        } else {
                            return Err(s);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn get_total_mass(&self) -> units::Gram {
        let mut r_mass: units::Gram = 0f64;
        for array in self.fuel_elements.iter() {
            for element in array.iter() {
                r_mass += element.fuel_mass;
            }
        }
        r_mass
    }

    pub fn get_temperature(&self) -> units::Kelvin {
        let mut temp: units::Kelvin = 0f64;
        for array in self.fuel_elements.iter() {
            for element in array.iter() {
                temp += element.temperature;
            }
        }
        let total: f64 = temp / (self.fuel_elements.len() as f64 * self.fuel_elements.len() as f64);
        total
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
