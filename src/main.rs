pub mod constants;
mod logger;
mod station;
mod control;
mod reactor;

use logger::init_the_loggers;

fn main() {
    init_the_loggers();
    
    println!("{:?}", constants::U_PROPERTIES);
}
