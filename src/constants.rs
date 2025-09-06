//! This module carries the public types for scientific units used and the `MaterialProperties` consts of every material modelled in the program.

/// Custom type for temperature
pub type Kelvin = f64;

/// Custom type for pressure
pub type Pascal = f64;

/// Pub type for mass
pub type Gram = f64;

/// grams per cubic cm (density)
pub type GramCm3 = f64;

/// For heat capacities of materials
pub type JoulesPerGramKelvin = f64;

/// Freezing point of water, in Kelvin at STP
pub const ZERO_CENTIGRADE: Kelvin = 273.15;

/// Room temperature (20C), in Kelvin at STP
pub const ROOM_TEMPERATURE: Kelvin = ZERO_CENTIGRADE + 20f64;

#[derive(Debug)]
/// Commonly used properties of materials involved
pub struct MaterialProperties {
  /// Melting point of the element/compound, in Kelvin
  pub melting_point: Kelvin,

  /// Boiling point of the element/compound, in Kelvin
  pub boiling_point: Kelvin,

  /// Density, in grams per centimetre cubed 
  pub density: GramCm3,

  /// Specific heat capacity, Joules of energy needed to increase the temperature of 1 gram of the material by 1 kelvin. 
  pub heat_capacity: JoulesPerGramKelvin,
}

/// Properties of the main fuel elements (uranium) in the reactor
pub const U_PROPERTIES: MaterialProperties = MaterialProperties {
  melting_point: 1135f64 + ZERO_CENTIGRADE,
  boiling_point: 4130f64 + ZERO_CENTIGRADE,
  density: 19.1f64,
  heat_capacity: 0.12f64,
};

/// Material properties for the coolant (regular H2O)
pub const H2O_PROPERTIES: MaterialProperties = MaterialProperties {
  melting_point: ZERO_CENTIGRADE,
  boiling_point: ZERO_CENTIGRADE + 100f64,
  density: 1f64,
  heat_capacity: 4.181f64,
};

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn uranium_constants() {
    assert_eq!(U_PROPERTIES.melting_point, 1408.15f64);
    assert_eq!(U_PROPERTIES.boiling_point, 4403.15f64);
    assert_eq!(U_PROPERTIES.density, 19.1f64);
    assert_eq!(U_PROPERTIES.heat_capacity, 0.12f64);
  }

  #[test]
  fn h2o_constants() {
    assert_eq!(H2O_PROPERTIES.boiling_point, 373.15f64);
  }
}
