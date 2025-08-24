# rusty-reactor

> This project is still in active development and as such is currently **not functional** and will not be 'generally available' for others to play about with until version `1.0.0`.  Please check the [Roadmap](https://github.com/orgs/West-DevOps/projects/4/views/2) for details about upcoming features and fixes but as this is a personal learning project *there is no release date* for `1.0.0`.

Playaround & learning rust by modelling (not scientifically at all) a nuclear power plant. 

Based around the `rand` crate to determine nuclear decay primarily 

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
* Once threaded might look into modelling real-world constraints e.g. commanding a SCRAM of the core doesn't happen instantly. 

## Program flow

* `main` thread performs CLI argument parsing using `clap`
* `main` thread inits the logging system.
* `main` thread creates the `Station` struct which has everything in it you would expect:
  * The `Core` struct
  * Coolant Loops
  * Control Room (SCADA & Operator command execution)
  * Generator and Turbine
  * etc.
* `main` thread starts the `station` thread
* `main` thread starts the user `cli` thread and sets up the channels between them
* `main` thread enters a wait state for either thread to bomb out and will panic the program
* `station` thread calls the `decay()` function on the `Core` and then checks for a user command from the `cli` thread in a `loop {}`. 
* `cli` thread attaches to the stdio handles and presents the CLI to the user / handles user interactions with the reactor program and dispatches those commands to the `station` thread using channels when the user executes a command.
