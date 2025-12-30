#[warn(missing_docs)]
pub mod units;
pub mod constants;
pub mod control;
pub mod reactor;

use std::sync::mpsc::channel;
use std::thread;
use clap::Parser;
use control::{ControlRoom, cli::CliArgs};
use reactor::Reactor;

/// Create the thread channels, reactor and control threads, wait for them to join.
fn main() {
    // Init functions
    let args = CliArgs::parse();
    
    // Create the channels the two threads will use to communicate.
    let (cli_tx, cli_rx) = channel();
    let (reactor_tx, reactor_rx) = channel();

    // Create and start the reactor thread.
    let reactor_thread_handle = thread::Builder::new().name("reactor-thread".into()).spawn(move || {
        let mut reactor: Reactor = Reactor::new(args.fuel_mass_per_element, args.exchanger_efficiency, args.panic_on_meltdown);
        let _ = reactor.start(cli_rx, reactor_tx);
    }).expect("Could not create the reactor thread");

    // start control room execution in this thread which is a blocking until either the reactor thread panics
    // or the user calls exit on the cli.
    let control_room: ControlRoom = ControlRoom::new(args.scada_sampling_interval, cli_tx, reactor_rx);
    
    match control_room.start() {
        Ok(_) => {},
        Err(msg) => {
            panic!("{}", msg);
        },
    }

    // Control thread has exited fine, the reactor thread _should_ be finished.
    if reactor_thread_handle.is_finished() {
        return // all good, exit 0
    }
    
    // If we get here the control room thread has exited before the reactor thread
    // This is unexpected behaviour:
    // The control room should kill the reactor thread before its start function returns Ok(())
    // But stdio errors in control room could lead us here. 
    // future development should solve this and ensure this code is unreachable. 
    println!("NOT GOOD!!! Control room thread exited and reactor still running!");
    
    match reactor_thread_handle.join() {
        Ok(_) => {},
        Err(error) => {
            panic!("{:?}", error);
        },
    }
}
