mod logger;
mod station;
mod turbine;
mod constants;
mod control;
mod reactor;

use logger::init_the_loggers;

fn main() {
    init_the_loggers();
    
}
