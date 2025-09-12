//! Deals with logging reactor state and dangerous condition detection/remediation. 
use crate::units;

/// Gathering and storing state of the cluster over time
pub struct Scada {
    core_temperature_history: Vec<units::Kelvin>,
    rod_position_history: Vec<units::RodPosition>,
    sampling_interval: units::Second,
}

impl Scada {
    /// Create a new SCADA object.
    pub(super) fn new(sampling_interval: units::Second) -> Scada {
        Scada {
            core_temperature_history: Vec::<units::Kelvin>::new(),
            rod_position_history: Vec::<units::RodPosition>::new(),
            sampling_interval: sampling_interval,
        }
    }

    /// Log the core temperature to the Vector
    pub(super) fn log_core_temp(&mut self, temperature: units::Kelvin) -> Result<units::Kelvin, String> {
        self.core_temperature_history.push(temperature);
        Ok(temperature)
    }

    /// Log the rod position to the appropriate Vector
    pub(super) fn log_rod_position(&mut self, position: units::RodPosition) -> Result<units::RodPosition, String> {
        self.rod_position_history.push(position);
        Ok(position)
    }

    /// Get the sampling interval from the SCADA system.
    pub(super) fn get_sample_interval(self) -> units::Second {
        self.sampling_interval
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vector_write() {
        let mut scada = Scada::new(1f64);
        scada.log_core_temp(350f64);
        scada.log_rod_position(23);
        assert!(true);
    }
}
