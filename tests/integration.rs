use rusty_reactor::station::Station;

#[test]
fn create_full_reactor_stack() {
    let station: Station = Station::new(50000.0, 80, 2);
    station.run();
}
