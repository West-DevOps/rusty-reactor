use std::io::{stderr, stdin, stdout, Stderr, Stdin, Stdout, Write};
use crate::{units, constants};

/// Prompt given to the user for each command
const PROMPT: &str = "Command> ";

#[derive(Debug, Clone, Copy)]
/// Gettable pieces of data from the Reactor
enum GetParams {
    Temperature,
    RemainingFuel,
    RodPosition
}

#[derive(Debug, Clone, Copy)]
/// The CoreCommand enum which is passed through the channels between 
/// `cli` and the `reactor`
pub enum CoreCommand {
    Scram,
    Get(GetParams),
    MoveRods(units::RodPosition),
}

/// Main code for the users cli thread
pub fn run_cli() {
    let sin: Stdin = stdin();
    let mut sout: Stdout = stdout();
    let serr: Stderr = stderr();

    loop {
        let _ = sout.lock();
        let _ = sout.write(PROMPT.as_bytes());
        let _ = sout.flush();

        let mut user_input_str = String::new();

        let _ = sin.read_line(&mut user_input_str);

        println!("{}", user_input_str);
    }
}
