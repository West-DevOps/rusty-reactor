use rusty_reactor::station;

#[test]
fn create_full_reactor_stack() {
    let _station = station::Station::new(50000.0, 80, 2);
}
