mod core;
mod coolant;

use crate::{control::cli::{CoreCommand, CoreResponse, GetParams}, units};
use core::Core;
use std::sync::mpsc::{Sender, Receiver};
use coolant::{Loop, Exchanger};

/// For grouping together the resources of the reactor, uses `start()` function to start the main `Core` `loop {}`.
pub struct Reactor {
    core: Core,
    primary_loop: Loop,
    secondary_loop: Loop,
    heat_exchanger: Exchanger,
}

impl Reactor {
    /// Create a new instance of the Reactor object / thread.
    pub fn new(fuel_load: units::Gram, exchanger_efficiency: units::Percent, panic_on_meltdown: bool) -> Reactor {
        Reactor {
            core: Core::new(fuel_load, panic_on_meltdown),
            primary_loop: Loop::new(),
            secondary_loop: Loop::new(),
            heat_exchanger: Exchanger::new(exchanger_efficiency),
        }
    }
    
    /// Get the secondary loop object. 
    pub fn get_heat_exchanger(self) -> Exchanger {
        self.heat_exchanger
    }
    
    /// Get the primary loop object. 
    pub fn get_primary_loop(self) -> Loop {
        self.primary_loop
    }

    /// Get the secondary loop object. 
    pub fn get_secondary_loop(self) -> Loop {
        self.secondary_loop
    }

    // Main reactor thread code.  Started by `main.rs`
    pub(super) fn start(&mut self, command_receiver: Receiver<CoreCommand>, response_sender: Sender<CoreResponse>) -> Result<(), String> {
        loop {
            // Call decay on the core
            match self.core.decay() {
                Ok(_) => {},
                Err(e) => {
                    let _ = response_sender.send(CoreResponse::Error(e.clone()));
                    return Err(e);
                },
            }
            
            // Check for CoreCommand on the receiver channel
            match command_receiver.try_recv() {
                Ok(cmd) => {
                    match cmd {
                        CoreCommand::Scram => {
                            let _ = self.core.set_rod_position(0u8);
                            let _ = response_sender.send(CoreResponse::Ok);
                        },
                        CoreCommand::Get(get_params) => {
                            match get_params {
                                GetParams::Temperature => {
                                    let _ = response_sender.send(CoreResponse::Temperature(self.core.get_temperature()));
                                },
                                GetParams::RemainingFuel => {
                                    let _ = response_sender.send(CoreResponse::RemainingFuel(self.core.get_total_mass()));
                                },
                                GetParams::RodPosition => {
                                    let _ = response_sender.send(CoreResponse::RodPosition(self.core.get_rod_position()));
                                },                   
                            }
                        },
                        CoreCommand::MoveRods(rpos) => {
                            let _ = self.core.set_rod_position(rpos);
                            let _ = response_sender.send(CoreResponse::Ok);
                        },
                        CoreCommand::Exit => todo!(),
                    }
                }, 
                Err(e) => {
                    match e {
                        std::sync::mpsc::TryRecvError::Empty => {},
                        std::sync::mpsc::TryRecvError::Disconnected => {
                            let response: String = String::from("CoreCommand receiver channel has disconnected, must exit.");
                            let _ = response_sender.send(CoreResponse::Error(response.clone()));
                            return Err(response);
                        },
                    }
                },
            }
        }
    }
}
