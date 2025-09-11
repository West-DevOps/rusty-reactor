use std::{fmt::Display, io::{stderr, stdin, stdout, Stderr, Stdin, Stdout, Write}, str::FromStr};
use crate::units;
use clap::Parser;

/// Prompt given to the user for each command
const PROMPT: &'static str = "reactor-cli $ ";


#[derive(Parser, Copy, Clone, Debug)]
/// Clap struct for programs CLI args.
pub struct CliArgs {
    #[arg(short, long, default_value = "5000.0")]
    pub fuel_mass_per_element: units::Gram,

    #[arg(short, long, default_value = "true")]
    pub panic_on_meltdown: bool,

    #[arg(short, long, default_value = "75")]
    pub exchanger_efficiency: units::Percent,

    #[arg(short, long, default_value = "1")]
    pub scada_sampling_interval: units::Second,
}

#[derive(Debug, Clone, Copy)]
/// Gettable pieces of data from the Reactor
pub(crate) enum GetParams {
    Temperature,
    RemainingFuel,
    RodPosition
}

#[derive(Debug, Clone, Copy)]
/// The CoreCommand enum which is passed through the channels between 
/// `cli` and the `reactor`
pub(crate) enum CoreCommand {
    Scram,
    Get(GetParams),
    MoveRods(units::RodPosition),
    Exit,
}

impl FromStr for CoreCommand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "exit" {
            Ok(CoreCommand::Exit)
        } else if s == "scram" {
            Ok(CoreCommand::Scram)
        } else if s.starts_with("get ") {
            // We need to some sub-scanning for get commands
            let getcmd = s.split(' ').next_back().unwrap();

            if getcmd == "temp" {
                Ok(CoreCommand::Get(GetParams::Temperature))
            } else if getcmd == "fuel" {
                Ok(CoreCommand::Get(GetParams::RemainingFuel))
            } else if getcmd == "rp" {
                Ok(CoreCommand::Get(GetParams::RodPosition))
            } else {
                return Err(String::from("Invalid Get Command"));
            }
        } else if s.starts_with("rpos ") {
            // MoveRods
            let rp = s.split(' ').next_back().unwrap();

            // Attempt to parse the users desired mr arg
            match rp.parse::<u8>() {
                Ok(int) => return Ok(CoreCommand::MoveRods(int)),
                Err(pie) => {
                    return Err(pie.to_string());
                },
            };
        } else {
            let mut message: String = String::from("Invalid Command: ");
            message.push_str(s);
            return Err(message);
        }
    }
}

impl Display for CoreCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoreCommand::Scram => write!(f, "Reactor Scrammed!"),
            CoreCommand::Get(get_params) => write!(f, "Get{:?}", get_params),
            CoreCommand::MoveRods(p) => write!(f, "{}% withdrawn", p),
            CoreCommand::Exit => write!(f, "Exit"),
        }
    }
}

#[derive(Debug, Clone)]
/// The CoreResponse enum which is passed through the channels between 
/// `Reactor` and the `ControlRoom`
pub(crate) enum CoreResponse {
    Ok,
    Warning(String),
    Error(String),
    Temperature(units::Kelvin),
    RodPosition(units::RodPosition),
    RemainingFuel(units::Gram),
}

impl Display for CoreResponse {
    /// Implement to_string() for CoreResponses
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoreResponse::Ok => write!(f, "Ok"),
            CoreResponse::Temperature(t) => write!(f, "{}K", t),
            CoreResponse::RodPosition(p) =>  write!(f, "{}% withdrawn", p),
            CoreResponse::RemainingFuel(r) => write!(f, "{}K", r),
            CoreResponse::Warning(message) => write!(f, "{}", message),
            CoreResponse::Error(message) => write!(f, "{}", message),
        }
    }
}

/// Send the program name, version and first user prompt to the stdout handle
pub(super) fn init() -> Result<(), String> {
    let mut sout: Stdout = stdout();

    let hello_message: String = format!("Welcome to {} version {}.\n", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    match sout.write_all(hello_message.as_ref()) {
        Ok(_) => {},
        Err(_) => return Err(String::from("Cannot write to stdout")),
    }

    match sout.flush() {
        Ok(_) => {},
        Err(_) => return Err(String::from("Cannot flush stdout")),
    }
    Ok(())
}

/// Read a command from stdin, (blocking function)
pub(super) fn read_command() -> Result<CoreCommand, String> {
    let sin: Stdin = stdin();
    let mut sout: Stdout = stdout();
    let user_input: &mut String = &mut "".into();

    let msg = format!("{}", PROMPT);
    let _ = sout.write(msg.as_ref());
    let _ = sout.flush();

    // This line locks the thread calling it and waits for a
    // entire line of input from stdin
    match sin.read_line(user_input) {
        Ok(_) => {},
        Err(_) => return Err("Could not read stdin".into()),
    }

    let stripped_cmd = user_input.strip_suffix("\n").unwrap();

    match CoreCommand::from_str(stripped_cmd) {
        Ok(cmd) => {
            return Ok(cmd)
        },
        Err(e) => {
            return Err(e);
        },
    }
}

/// Write a message to the stdout handle
pub(super) fn write_response(message: String) -> Result<(), String> {
    let mut sout: Stdout = stdout();

    match sout.write_all(message.as_ref()) {
        Ok(_) => {},
        Err(error) => return Err(error.to_string()),
    }
    

    match sout.flush() {
        Ok(_) => {},
        Err(error) => return Err(error.to_string()),
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ok_response() {
        assert_eq!(CoreResponse::Ok.to_string(), String::from("Ok"));
    }

    #[test]
    fn core_command_from_str_exit() {
        let cmd = CoreCommand::from_str("exit").expect("Cannot from_str");
        match cmd {
            CoreCommand::Exit => { assert!(true) },
            _ => { assert!(false) }
        }
    }

    #[test]
    fn core_command_from_str_get_temp() {
        let cmd = CoreCommand::from_str("get temp").expect("Cannot from_str");
        match cmd {
            CoreCommand::Get(GetParams::Temperature) => { assert!(true) },
            _ => { assert!(false) }
        }
    }

    #[test]
    fn core_command_from_str_move_rod() {
        let cmd = CoreCommand::from_str("rpos 50").expect("Cannot from_str");
        match cmd {
            CoreCommand::MoveRods(50) => { assert!(true) },
            _ => { assert!(false) }
        }
    }
}
