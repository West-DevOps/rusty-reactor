mod scada;
pub mod cli;

use scada::Scada;

/// Represents the command and control side of the reactor sim
/// It is concerned with:
/// * Running the CLI thread
/// * Logging reactor state (temps, pressures etc.)
/// * Detecting dangerous conditions and auto-regulation of reactor 
pub struct ControlRoom {
    scada: Scada,
}

impl ControlRoom {
    pub fn new() -> ControlRoom {
        ControlRoom {
            scada: Scada::new(1f64),
        }
    }

}
