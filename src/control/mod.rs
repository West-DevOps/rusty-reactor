mod scada;
pub mod cli;

use scada::Scada;

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
