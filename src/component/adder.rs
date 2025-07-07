use crate::{
    circuit::{ANDGate, ORGate, Potential, Wire, XORGate},
    component::Component,
};

/// a half adder in circuite.
/// the input is 2 bits, and the output is 2 bits.
///
/// ```ascii
///       ┌────────────────┐          
/// A─────┼                ┼─────Carry
///       │   Half Adder   │          
/// B─────┼                ┼─────Sum  
///       └────────────────┘             
/// ```
/// # output   
/// the first bit is the carry bit, and the second bit is the sum bit.
/// the sum bit is the xor of the two bits.
/// the carry bit is the and of the two bits.
///
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
        // Sum
        self.output[0].input(&self.xor_gate.output());
        // Carry
        self.output[1].input(&self.and_gate.output());
    }
}

/// a full adder in circuite.
/// the input is 3 bits, and the output is 2 bits.
///
/// ```ascii
///       ┌────────────────┐                    ┌────┐        
/// A─────┼A              C┼────────────────────┼    │        
///       │   Half Adder   │                    │ OR ┼───Carry
/// B─────┼B              S┼────┐             ┌─┼    │        
///       └────────────────┘    │             │ └────┘        
///                             │             │               
///                   ┌─────────┘             │               
///                   │    ┌────────────────┐ │               
///                   ┴────┼A              C┼─┘               
///                        │   Half Adder   │                 
/// Carry──────────────────┼B              S┼────────────Sum  
///                        └────────────────┘                 
///```
/// # input
/// the last bit is carry from another adder
///
/// # output
/// the first bit is the carry bit, and the second bit is the sum bit.

#[derive(Debug, Default, Clone)]
struct FullAdder {
    half_adder: [HalfAdder; 2],
    or_gate: ORGate,
    input: [Wire; 3],
    output: [Wire; 2],
}
impl Component for FullAdder {
    fn get_pin_count(&self) -> (usize, usize) {
        (3, 2)
    }
    fn set_pin_input(&mut self, position: usize, value: &Potential) {
        assert!(
            position < self.get_pin_count().0,
            "position must be less than {}",
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
        self.half_adder[0].fire(&vec![self.input[0].output(), self.input[1].output()]);
        let out1 = self.half_adder[0].output();
        self.half_adder[1].fire(&vec![out1[0], self.input[2].output()]);
        let out2 = self.half_adder[1].output();
        self.or_gate.input(&out1[1], &out2[1]);
        // Little-Endian
        // Sum
        self.output[0].input(&out2[0]);
        // Carry
        self.output[1].input(&self.or_gate.output());
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
    #[test]
    fn test_full_adder_default() {
        let full_adder = FullAdder::default();
        assert_eq!(full_adder.output(), vec![false, false]);
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

    #[rstest]
    #[case(false, false, false, false, false)]
    #[case(false, false, true, true, false)]
    #[case(false, true, false, true, false)]
    #[case(false, true, true, false, true)]
    #[case(true, false, false, true, false)]
    #[case(true, false, true, false, true)]
    #[case(true, true, false, false, true)]
    #[case(true, true, true, true, true)]
    fn test_full_adder_input(
        #[case] a: bool,
        #[case] b: bool,
        #[case] c: bool, // carry bit
        #[case] d: bool, // sum bit
        #[case] e: bool, // carry bit
    ) {
        let mut full_adder = FullAdder::default();
        full_adder.input(&vec![a, b, c]);
        full_adder.update_state();
        assert_eq!(full_adder.output(), vec![d, e]);
    }
}
