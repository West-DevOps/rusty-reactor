mod scada;
pub mod cli;

use scada::Scada;

struct ControlRoom {
    scada: Scada,
}
