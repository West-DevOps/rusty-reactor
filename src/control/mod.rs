//! Command and control code for the reactor
mod scada;
pub mod cli;

use scada::Scada;
use crate::units;

/// Represents the command and control side of the reactor sim
/// It is concerned with:
/// * Running the CLI thread
/// * Logging reactor state (temps, pressures etc.)
/// * Detecting dangerous conditions and auto-regulation of reactor 
pub struct ControlRoom {
    scada: Scada,
}

impl ControlRoom {
    /// Create new instance of the ControlRoom, the top-level object for this module
    pub fn new() -> ControlRoom {
        ControlRoom {
            scada: Scada::new(1f64),
        }
    }

    pub fn get_scada_sampling_interval(self) -> units::Second {
        self.scada.get_sample_interval()
    }
}

#[cfg(test)]
mod test {
    use super::*; 

    #[test]
    fn test_new_construct() {
        assert_eq!(ControlRoom::new().get_scada_sampling_interval(), 1f64);
    }
}
