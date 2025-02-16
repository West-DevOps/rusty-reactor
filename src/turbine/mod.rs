mod turbine;
mod generator;

use turbine::SteamTurbine;
use generator::ElectricalGenerator;
use super::reactor::coolant::Loop;

pub struct TurbineHall {
    steam_turbine: SteamTurbine,
    generator: ElectricalGenerator,
}

impl TurbineHall {
    pub fn new(generator_ratio: u8) -> TurbineHall {
        TurbineHall {
            steam_turbine: SteamTurbine::new(),
            generator: ElectricalGenerator::new(generator_ratio),
        }
    }
}
