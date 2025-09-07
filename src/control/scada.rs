use crate::units;

pub struct Scada {
    core_temperature_history: Vec<units::Kelvin>,
    rod_position_history: Vec<units::RodPosition>,
    sampling_interval: units::Second,
}

impl Scada {
    pub fn new(sampling_interval: units::Second) -> Scada {
        Scada {
            core_temperature_history: Vec::<f64>::new(),
            rod_position_history: Vec::<u8>::new(),
            sampling_interval: sampling_interval,
        }
    }

    pub fn log_core_temp(&mut self, temperature: units::Kelvin) -> Result<units::Kelvin, String> {
        self.core_temperature_history.push(temperature);
        Ok(temperature)
    }

        pub fn log_rod_position(&mut self, position: units::RodPosition) -> Result<units::RodPosition, String> {
        self.rod_position_history.push(position);
        Ok(position)
    }
}
