//! Command and control code for the reactor
mod scada;
pub mod cli;

use std::sync::mpsc::{Sender, Receiver};
use scada::Scada;
use crate::{control::cli::{CoreCommand, CoreResponse}, units};

/// Represents the command and control side of the reactor sim
/// It is concerned with:
/// * Reading and Writing the CLI.
/// * Logging reactor state (temps, pressures etc.)
/// * Detecting dangerous conditions and auto-regulation of reactor 
pub struct ControlRoom {
    scada: Scada,
    core_sender: Sender<CoreCommand>,
    core_receiver: Receiver<CoreResponse>,
}

impl ControlRoom {
    /// Create new instance of the ControlRoom, the top-level object for this module
    pub fn new(scada_sampling_interval: units::Second, core_sender: Sender<CoreCommand>, core_receiver: Receiver<CoreResponse>) -> ControlRoom {
        ControlRoom {
            scada: Scada::new(scada_sampling_interval),
            core_sender,
            core_receiver,
        }
    }

    pub fn get_scada_sampling_interval(self) -> units::Second {
        self.scada.get_sample_interval()
    }

    /// Called by `main.rs` to make the main thread this ControlRoom 
    pub(super) fn start(&self) -> Result<(), String> {
        match cli::init_cli() {
            Ok(_) => {},
            Err(msg) => { return Err(msg) },
        }



        // Here we enter the ControlRooms eternal loop
        loop {
            
        }
    }
}

