use crate::{
    circuit::{ANDGate, NOTGate, ORGate, Potential, Wire},
    component::{Component, big_gates::ORGate3},
};

/// 2-1 Simple Encoder.
/// 
/// # description
/// [Simple Encoder](https://en.wikipedia.org/wiki/Encoder_(digital)) convert 2 bits to 1 bit.
/// 
/// # truth table
/// | I0 | I1 | OUT |
/// |---|---|---|
/// | 1 | 0 | 0 |
/// | 0 | 1 | 1 |
#[derive(Debug, Default, Clone)]
struct Encoder2_1 {
    input: [Wire; 2],
    output: [Wire; 1],
}

impl Component for Encoder2_1 {
    fn get_pin_count(&self) -> (usize, usize) {
        (2, 1)
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
        self.output[0].input(&self.input[1].output());
    }
}

/// 4-2 Simple Encoder.
/// 
/// # description
/// [Simple Encoder](https://en.wikipedia.org/wiki/Encoder_(digital)) convert 4 bits to 2 bit.
/// 
/// # truth table
/// | I0 | I1 | I2 | I3 | OUT0 | OUT1 |
/// |---|---|---|---|---|---|
/// | 1 | 0 | 0 | 0 | 0 | 0 |
/// | 0 | 1 | 0 | 0 | 1 | 0 |
/// | 0 | 0 | 1 | 0 | 0 | 1 |
/// | 0 | 0 | 0 | 1 | 1 | 1 |
/// # input
///
/// ```mermaid
///  ---
///  title: "input Packet"
///  ---
///  packet-beta
///  0: "I0"
///  1: "I1"
///  2: "I2"
///  3: "I3"
/// ```
///
/// # output
///
/// ```mermaid
///  ---
///  title: "out Packet"
///  ---
///  packet-beta
///  0: "out0"
///  1: "out1"
/// ```
#[derive(Debug, Default, Clone)]
struct Encoder4_2 {
    input: [Wire; 4],
    output: [Wire; 2],
    or_gates: [ORGate; 2],
}

impl Component for Encoder4_2 {
    fn get_pin_count(&self) -> (usize, usize) {
        (4, 2)
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
        self.or_gates[0].input(&self.input[3].output(), &self.input[1].output());
        self.or_gates[1].input(&self.input[3].output(), &self.input[2].output());
        self.output[0].input(&self.or_gates[0].output());
        self.output[1].input(&self.or_gates[1].output());
    }
}

/// 4-2 Priority Encoder.
///
/// # description
/// 4-2 [Priority Encoder](https://en.wikipedia.org/wiki/Priority_encoder) is a 4-2 Encoder with priority.
/// The priority is I3 > I2 > I1 > I0.
///
/// # truth table
///
/// | I0 | I1 | I2 | I3 | OUT0 | OUT1 | v |
/// |---|---|---|---|---|---|---|
/// | 0 | 0 | 0 | 0 | any | any | 0 |
/// | 1 | 0 | 0 | 0 | 0 | 0 | 1 |
/// | any | 1 | 0 | 0 | 1 | 0 | 1 |
/// | any | any | 1 | 0 | 0 | 1 | 1 |
/// | any | any | any | 1 | 1 | 1 | 1 |
///
/// 1. any means input 0 or 1.
/// 2. v means wheather the input is valid.
///
/// # input
///
/// ```mermaid
///  ---
///  title: "input Packet"
///  ---
///  packet-beta
///  0: "I0"
///  1: "I1"
///  2: "I2"
///  3: "I3"
/// ```
///
/// # output
///
/// ```mermaid
///  ---
///  title: "out Packet"
///  ---
///  packet-beta
///  0: "out0"
///  1: "out1"
///  2: "v"
/// ```
///
#[derive(Debug, Default, Clone)]
struct PriorityEncoder4_2 {
    input: [Wire; 4],
    output: [Wire; 3],
    or_gate_1: ORGate,
    or_gate_2: ORGate,
    and_gate: ANDGate,
    not_gate: NOTGate,
    big_or: ORGate3,
}

impl Component for PriorityEncoder4_2 {
    fn get_pin_count(&self) -> (usize, usize) {
        (4, 3)
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
        self.or_gate_1
            .input(&self.input[3].output(), &self.input[2].output());
        let o1 = &self.or_gate_1.output();
        self.output[1].input(o1);

        self.not_gate.input(&self.input[2].output());
        self.and_gate
            .input(&self.input[1].output(), &self.not_gate.output());
        self.or_gate_2
            .input(&self.input[3].output(), &self.and_gate.output());
        let o0 = &self.or_gate_2.output();
        self.output[0].input(o0);
        self.big_or.input(&vec![
            o1.to_owned(),
            self.input[0].output(),
            self.input[1].output(),
        ]);
        // v
        self.output[2].input(&self.big_or.output()[0]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_encoder2_1_default() {
        let enabler = Encoder2_1::default();
        assert_eq!(enabler.output(), vec![false]);
    }
    #[rstest]
    #[case(vec![true,false],vec![false])]
    #[case(vec![false,true],vec![true])]
    fn test_encoder2_1_truth_table(
        #[case] input: Vec<Potential>,
        #[case] expected: Vec<Potential>,
    ) {
        let mut enabler = Encoder2_1::default();
        enabler.input(&input);
        assert_eq!(enabler.output(), expected);
    }
    #[test]
    fn test_encoder4_2_default() {
        let enabler = Encoder4_2::default();
        assert_eq!(enabler.output(), vec![false, false]);
    }
    #[rstest]
    #[case(vec![true,false,false,false],vec![false,false])]
    #[case(vec![false,true,false,false],vec![true,false])]
    #[case(vec![false,false,true,false],vec![false,true])]
    #[case(vec![false,false,false,true],vec![true,true])]
    fn test_encoder4_2_truth_table(
        #[case] input: Vec<Potential>,
        #[case] expected: Vec<Potential>,
    ) {
        let mut enabler = Encoder4_2::default();
        enabler.input(&input);
        assert_eq!(enabler.output(), expected);
    }
    #[test]
    fn test_priority_encoder4_2_default() {
        let enabler = PriorityEncoder4_2::default();
        assert_eq!(enabler.output(), vec![false, false, false]);
    }
    #[rstest]
    #[case(vec![false,false,false,false],vec![false,false,false])]
    #[case(vec![true,false,false,false],vec![false,false,true])]
    #[case(vec![false,true,false,false],vec![true,false,true])]
    #[case(vec![false,false,true,false],vec![false,true,true])]
    #[case(vec![false,false,false,true],vec![true,true,true])]
    fn test_priority_encoder4_2_truth_table(
        #[case] input: Vec<Potential>,
        #[case] expected: Vec<Potential>,
    ) {
        let mut enabler = PriorityEncoder4_2::default();
        enabler.input(&input);
        assert_eq!(enabler.output(), expected);
    }
}
