pub mod units;
pub mod constants;
mod control;
mod reactor;

/// Create the station and control threads, wait for them to join.
fn main() {
    println!("{:?}", constants::U_PROPERTIES);
    println!("{:?}", constants::H2O_PROPERTIES);
}
