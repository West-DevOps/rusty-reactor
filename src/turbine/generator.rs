pub struct ElectricalGenerator {
    // 1rpm = 1Watt of power
    // Box ratio of 2 would mean doubling of ouput power (and required torque to spin it)
    input_shaft_rpm: f32,
    box_ratio: u8,
    output_power: f32,
}

impl ElectricalGenerator {
    pub fn new(ratio: u8) -> ElectricalGenerator {
        ElectricalGenerator {
            input_shaft_rpm: 0.0,
            box_ratio: ratio,
            output_power: 0.0,
        }
    }

    pub fn get_output_power(&self) -> f32 {
        self.input_shaft_rpm * self.box_ratio as f32
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gear_reduction() {
        let mut generator: ElectricalGenerator = ElectricalGenerator::new(2);
        generator.input_shaft_rpm = 1000.0;

        assert_eq!(generator.get_output_power(), 2000.0);
    }
}