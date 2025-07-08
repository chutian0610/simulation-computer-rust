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
/// the first bit is the sum bit, and the second bit is the carry bit.
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
/// the first 1 bit is A , the next 1 bit is B and the last bit is carry from another adder
///
/// # output
/// the first bit is the sum bit, and the second bit is the carry bit.

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
#[cfg_attr(doc, aquamarine::aquamarine)]
/// a ripple carry adder in circuite.
/// the input is 2*n+1 bits, and the output is n+1 bits.
///
/// [4 bit ripple carry adder example](https://upload.wikimedia.org/wikipedia/commons/5/5d/4-bit_ripple_carry_adder.svg)
/// 
/// # input
/// the first 1 bit is Carry from another adder, the next n bit is A and the last N bit is B 
/// 
/// ```mermaid
///  ---
///  title: "input Packet"
///  ---
///  packet-beta
///  0: "carry"
///  1: "a0"
///  2: "a1"
///  3: "a2"
///  4: "a3"
///  5: "b0"
///  6: "b1"
///  7: "b2"
///  8: "b3"
/// ```
///
/// # output
/// the first n bit is the sum bit, and the next 1 bit is the carry bit.
/// ```mermaid
///  ---
///  title: "output Packet"
///  ---
///  packet-beta
///  0: "s0"
///  1: "s1"
///  2: "s2"
///  3: "s3"
///  4: "carry"
/// ```
#[derive(Debug, Clone)]
struct RippleCarryAdder {
    n_way: usize,
    input: Vec<Wire>,
    full_adders: Vec<FullAdder>,
    output: Vec<Wire>,
}

impl RippleCarryAdder {
    fn new(n_way: usize) -> Self {
        Self {
            n_way,
            input: vec![Wire::default(); 2*n_way+1],
            full_adders: vec![FullAdder::default(); n_way],
            output: vec![Wire::default(); n_way + 1]
        }
    }
}

impl Component for RippleCarryAdder {
    fn get_pin_count(&self) -> (usize, usize) {
        (2*self.n_way+1,self.n_way+1)
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
        // the fist full adder's carry bit is the carry bit from another adder
        self.full_adders[0].fire(&vec![
            // first bit of A 
            self.input[1].output(),
            // first bit of B
            self.input[1+self.n_way].output(),
            // carry
            self.input[0].output()
        ]);
        // cursor = (sum,carry)
        let mut cursor = self.full_adders[0].output();
        for i in 1..self.n_way {
            self.output[i-1].input(&cursor[0]);
            self.full_adders[i].fire(&vec![
                self.input[1+i].output(),
                self.input[1+self.n_way+i].output(),
                // carry
                cursor[1]
            ]);
            // update cursor
            cursor = self.full_adders[i].output();

        }
        // last sum
        self.output[self.n_way-1].input(&cursor[0]);
        // last carry
        self.output[self.n_way].input(&cursor[1]);
    }
}

#[cfg(test)]
mod tests {
    use crate::{circuit::Potentials, component::adder};

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

    #[test]
    fn test_ripple_carry_adder_default() {
        let adder_4 = RippleCarryAdder::new(4);
        assert_eq!(adder_4.output(), vec![false, false,false,false,false]);
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

    #[rstest]
    /// carry | a | b  => sum | carry
    #[case("0 00 00","00 0")]
    #[case("0 10 00","10 0")]
    #[case("0 10 10","01 0")]
    #[case("0 11 10","00 1")]
    #[case("0 11 11","01 1")]
    #[case("1 00 00","10 0")]
    #[case("1 10 00","01 0")]
    #[case("1 10 10","11 0")]
    #[case("1 11 10","10 1")]
    #[case("1 11 11","11 1")]
    fn test_ripple_carry_adder_input(#[case] input:String,#[case] output:String) {
        let mut adder_2 = RippleCarryAdder::new(2);
        let i: Potentials = Potentials::from_little_endian(&input, false);
        adder_2.fire(&i.get_data(true));
        let o = Potentials::from_little_endian(&output, false);
        assert_eq!(adder_2.output(), o.get_data(true));
    }
}
