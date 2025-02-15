mod core;

fn main() {
    let mut core = core::build_core(99999.0);

    let _ = core.withdraw_rods(10);

    let mut count: u32 = 0;
    let stop_count: u32 = 9999999;

    loop {
        match core.decay() {
            Ok(_) => {},
            Err(s) => {
                let _ = core.scram();
                println!("Cycles = {}, Error = {}", count, s);
                break;
            }
        }

        count += 1;

        if core.get_temperature() > 1200.0 && count % 10 == 0 {
            match core.insert_rods(1) {
                Ok(_) => {},
                Err(s) => {
                    println!("{}", s);
                },
            }
        } else if core.get_temperature() < 500.0 && count % 10 == 0 {
            match core.withdraw_rods(1) {
                Ok(_) => {},
                Err(s) => {
                    println!("{}", s);
                },
            }
        }

        if count % 1000 == 0 {
            println!("Count: {} Rods: {}%, Temp: {}K, Mass: {}g", count, core.get_rod_position(), core.get_temperature(), core.get_u_mass());
        }
        
        if count == stop_count {
            break;
        }
    }

    println!("Final core state:");
    println!("Count: {} Rods: {}%, Temp: {}K, Mass: {}g", count, core.get_rod_position(), core.get_temperature(), core.get_u_mass());
}
