
pub struct Scada {
    core_temperature_history: Vec<f64>
}

impl Scada {
    pub fn new() -> Scada {
        Scada {
            core_temperature_history: Vec::<f64>::new(),
        }
    }

    pub fn log_core_temp(&mut self, temperature: f64) -> Result<f64, String> {
        self.core_temperature_history.push(temperature);
        Ok(temperature)
    }
}
