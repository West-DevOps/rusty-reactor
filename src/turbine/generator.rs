pub struct ElectricalGenerator {
    input_shaft_rpm: f32,
    box_ratio: u8,
    output_power: f32,
}

impl ElectricalGenerator {
    pub fn new() -> ElectricalGenerator {
        ElectricalGenerator {
            input_shaft_rpm: 0.0,
            box_ratio: 1,
            output_power: 0.0,
        }
    }
}
