use crate::{
    circuit::{ANDGate, Potential, Wire, XORGate},
    component::Component,
};

/// a half adder in circuite.
/// the input is 2 bits, and the output is 2 bits.
///
/// # output   
/// the first bit is the carry bit, and the second bit is the sum bit.
/// the sum bit is the xor of the two bits.
/// the carry bit is the and of the two bits.
#[derive(Debug, Default, Clone)]
struct HalfAdder {
    input: [Wire; 2],
    output: [Wire; 2],
    and_gate: ANDGate,
    xor_gate: XORGate,
}

impl Component for HalfAdder {
    fn get_pin_count(&self) -> (usize, usize) {
        (2, 2)
    }
    fn set_pin_input(&mut self, position: usize, value: &Potential) {
        assert!(
            position < self.get_pin_count().0,
            "position  must be less than {}",
            self.get_pin_count().0
        );
        self.input[position].input(value);
    }

    fn get_pin_output(&self, position: usize) -> Potential {
        assert!(
            position < self.get_pin_count().1,
            "position must be less than {}",
            self.get_pin_count().1
        );
        self.output[position].output()
    }

    fn update_state(&mut self) {
        self.and_gate
            .input(&self.input[0].output(), &self.input[1].output());
        self.xor_gate
            .input(&self.input[0].output(), &self.input[1].output());
        // Little-Endian
        self.output[0].input(&self.xor_gate.output());
        self.output[1].input(&self.and_gate.output());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    #[test]
    fn test_half_adder_default() {
        let half_adder = HalfAdder::default();
        assert_eq!(half_adder.output(), vec![false, false]);
    }

    #[rstest]
    #[case(false, false, false, false)]
    #[case(false, true, true, false)]
    #[case(true, false, true, false)]
    #[case(true, true, false, true)]
    fn test_half_adder_input(
        #[case] a: bool,
        #[case] b: bool,
        #[case] c: bool, // sum bit
        #[case] d: bool,
    ) // carry bit
    {
        let mut half_adder = HalfAdder::default();
        half_adder.input(&vec![a, b]);
        half_adder.update_state();
        assert_eq!(half_adder.output(), vec![c, d]);
    }
}
