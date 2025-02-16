
pub struct Scada {
    core_temperature_history: Vec<f32>
}

impl Scada {
    pub fn new() -> Scada {
        Scada {
            core_temperature_history: Vec::<f32>::new(),
        }
    }

    pub fn log_core_temp(&mut self, temperature: f32) -> Result<f32, String> {
        self.core_temperature_history.push(temperature);
        Ok(temperature)
    }
}
