# rusty-reactor

Playaround & learning rust by modelling (not scientifically at all) a nuclear power plant. 

## What does each module do?

The modules (files) are laid out as you would expect a physical reactor station.  
This is probably not the most efficent or correct way to model this in the Rust language but it's what I have chosen! 

* Reactor containment structure (`reactor` module/dir) which houses the:
  * The `core` struct and implementation
  * The `coolant` loop implementation for both primary and secondary coolant loops 
  * a heat `Exchanger` between the two coolant loops 
  * Pre-heater for raising the primary coolant temperature
  * The coolant pumps and valves etc.
* Turbine hall (`turbine` module/dir) which contains:
  * The steam `Turbine` itself (rotation speed and mass on flywheel = stored kinetic energy / torque)
  * The turbine bypass valves (slows it down)
  * The `Generator` which puts a load on the turbine (slows it down)
* And finally the Control Room (`control` module/dir) which contains:
  * [SCADA](https://en.wikipedia.org/wiki/SCADA) implementation (main control system and data loggers)
    * This will work with the historical data (temperature logging into vectors etc.)
  * The CLI and UI componenets 

which will be used to learn how to solve problems with `Rust`... e.g.

* `Core` will explore using the [rand](https://crates.io/crates/rand/) crate to model (very basically) nuclear decay and heating
* Coolant `Loop` and `Exchanger` implementations will explore passing data (mostly changes in temperatures) around the code
* `Turbine` and `Generator` are again very basic implementations just for completeness 

In future:

* Use the various stages to do **harder** maths such as computing the _actual_ heat transfer of the coolant loops
* efficiency of heat exchange with larger/smaller heat exhangers between primary and secondary loops working with surface areas and heat capacities
* Modelling pressure and state changes in the secondary coolant loop (liquid water into steam and condensation etc.)
* Use the [thread](https://doc.rust-lang.org/std/thread/index.html) library to make this all work in a more realistic fashion! 
* Once threaded might look into modelling real-world constraints e.g. commanding a SCRAM of the core doesn't happen instantly. 
