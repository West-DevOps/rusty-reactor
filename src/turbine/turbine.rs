pub const FLYWHEEL_MASS: f32 = 50000.0; // 50 Ton in grams

pub struct SteamTurbine {
    rpm: f32,
    flywheel_mass: f32,
}

impl SteamTurbine {
    pub fn new() -> SteamTurbine {
        SteamTurbine {
            rpm: 0.0,
            flywheel_mass: FLYWHEEL_MASS,
        }
    }
}
