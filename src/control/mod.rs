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
    pub(crate) fn new(scada_sampling_interval: units::Second, core_sender: Sender<CoreCommand>, core_receiver: Receiver<CoreResponse>) -> ControlRoom {
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
        match cli::init() {
            Ok(_) => {},
            Err(msg) => { return Err(msg) },
        }

        loop {
            match self.core_receiver.try_recv() {
                Ok(response) => {
                    match cli::write_response(response.to_string()) {
                        Ok(_) => {},
                        Err(e) => { return Err(e) },
                    }
                },
                Err(e) => {
                    match e {
                        std::sync::mpsc::TryRecvError::Empty => {},
                        std::sync::mpsc::TryRecvError::Disconnected => {
                            match cli::write_response(String::from("Disconnected channel, must exit.")) {
                                Ok(_) => {},
                                Err(e) => { return Err(e) },
                            }
                        },
                    }
                },
            }

            match cli::read_command() {
                Ok(cmd) => {
                    match self.core_sender.send(cmd) {
                        Ok(_) => {},
                        Err(e) => { return Err(e.to_string()) },
                    }
                },
                Err(e) => { 
                    println!("{}", e);
                 },
            }            
        }
    }
}
