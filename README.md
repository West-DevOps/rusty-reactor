# rusty-reactor

> This project is still in active development and as such is currently **not functional** and will not be 'generally available' for others to play about with until version `1.0.0`.  Please check the [Roadmap](https://github.com/orgs/West-DevOps/projects/4/views/2) for details about upcoming features and fixes but as this is a personal learning project *there is no release date* for `1.0.0`.

Playaround & learning rust by modelling (not scientifically at all) a nuclear power plant. 

Based around the `rand` crate to determine nuclear decay primarily 

In future:

* Use the various stages to do **harder** maths such as computing the _actual_ heat transfer of the coolant loops
* efficiency of heat exchange with larger/smaller heat exhangers between primary and secondary loops working with surface areas and heat capacities
* Modelling pressure and state changes in the secondary coolant loop (liquid water into steam and condensation etc.)
* Once threaded might look into modelling real-world constraints e.g. commanding a SCRAM of the core doesn't happen instantly. 
