mod logger;
mod station;
mod turbine;
mod constants;
mod control;
mod reactor;

use logger::init_the_loggers;
use log::{log, Level};

fn main() {
    init_the_loggers();
    
}
