use crate::circuit::{ANDGate, ORGate, Potential, Wire};

use super::Component;

/// 3-input big AND gates
#[derive(Default)]
pub struct ANDGate3 {
    input: [Wire; 3],
    and_gate: [ANDGate; 2],
    output: Wire,
}

impl Component for ANDGate3 {
    fn get_pin_output(&self, position: usize) -> Potential {
        assert!(
            position < self.get_pin_count().1,
            "position must be less than  {}",
            self.get_pin_count().1
        );
        self.output.output()
    }

    fn set_pin_input(&mut self, position: usize, value: &Potential) {
        assert!(
            position < self.get_pin_count().0,
            "position must be less than {}",
            self.get_pin_count().0
        );
        self.input[position].input(&value);
    }

    fn update_state(&mut self) {
        self.and_gate[0].input(&self.input[0].output(), &self.input[1].output());
        self.and_gate[1].input(&self.and_gate[0].output(), &self.input[2].output());
        self.output.input(&self.and_gate[1].output());
    }

    fn get_pin_count(&self) -> (usize, usize) {
        (3, 1)
    }
}

/// 3-input big OR gates
#[derive(Default)]
pub struct ORGate3 {
    input: [Wire; 3],
    or_gate: [ORGate; 2],
    output: Wire,
}

impl Component for ORGate3 {
    fn get_pin_output(&self, position: usize) -> Potential {
        assert!(
            position < self.get_pin_count().1,
            "position must be less than  {}",
            self.get_pin_count().1
        );
        self.output.output()
    }

    fn set_pin_input(&mut self, position: usize, value: &Potential) {
        assert!(
            position < self.get_pin_count().0,
            "position must be less than {}",
            self.get_pin_count().0
        );
        self.input[position].input(&value);
    }

    fn update_state(&mut self) {
        self.or_gate[0].input(&self.input[0].output(), &self.input[1].output());
        self.or_gate[1].input(&self.or_gate[0].output(), &self.input[2].output());
        self.output.input(&self.or_gate[1].output());
    }

    fn get_pin_count(&self) -> (usize, usize) {
        (3, 1)
    }
}

/// N way-input big AND gates
pub struct ANDGateN {
    n_way: usize,
    input: Vec<Wire>,
    and_gate: Vec<ANDGate>,
    output: Wire,
}

impl ANDGateN {
    fn new(n_way: usize) -> Self {
        Self {
            n_way,
            input: vec![Wire::default(); n_way],
            and_gate: vec![ANDGate::default(); n_way - 1],
            output: Wire::default(),
        }
    }
}

impl Component for ANDGateN {
    fn get_pin_count(&self) -> (usize, usize) {
        (self.n_way, 1)
    }

    fn get_pin_output(&self, position: usize) -> Potential {
        assert!(
            position < self.get_pin_count().1,
            "position must be less than  {}",
            self.get_pin_count().1
        );
        self.output.output()
    }

    fn set_pin_input(&mut self, position: usize, value: &Potential) {
        assert!(
            position < self.get_pin_count().0,
            "position must be less than {}",
            self.get_pin_count().0
        );
        self.input[position].input(&value);
    }

    fn update_state(&mut self) {
        self.and_gate[0].input(&self.input[0].output(), &self.input[1].output());
        {}
        for i in 1..self.n_way - 1 {
            // use tmp variable avoid borrow problem
            let tmp_1 = &self.and_gate[i - 1].output();
            let tmp_2 = &self.input[i + 1].output();
            self.and_gate[i].input(tmp_1, tmp_2);
        }
        self.output.input(&self.and_gate[self.n_way - 2].output());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_and_gate_3_default() {
        let and_gate_3 = ANDGate3::default();
        assert_eq!(and_gate_3.output(), vec![false]);
    }

    #[rstest]
    #[case(true, true, true, true)]
    #[case(true, true, false, false)]
    #[case(true, false, true, false)]
    #[case(true, false, false, false)]
    #[case(false, true, true, false)]
    #[case(false, true, false, false)]
    #[case(false, false, true, false)]
    #[case(false, false, false, false)]
    fn test_and_gate_3_with_truth_table(
        #[case] a: bool,
        #[case] b: bool,
        #[case] c: bool,
        #[case] d: bool,
    ) {
        let mut and_gate_3 = ANDGate3::default();
        and_gate_3.input(&vec![a, b, c]);
        and_gate_3.update_state();
        assert_eq!(and_gate_3.output(), vec![d]);
    }

    #[test]
    fn test_or_gate_3_default() {
        let or_gate_3 = ORGate3::default();
        assert_eq!(or_gate_3.output(), vec![false]);
    }

    #[rstest]
    #[case(true, true, true, true)]
    #[case(true, true, false, true)]
    #[case(true, false, true, true)]
    #[case(true, false, false, true)]
    #[case(false, true, true, true)]
    #[case(false, true, false, true)]
    #[case(false, false, true, true)]
    #[case(false, false, false, false)]
    fn test_or_gate_3_with_truth_table(
        #[case] a: bool,
        #[case] b: bool,
        #[case] c: bool,
        #[case] d: bool,
    ) {
        let mut or_gate_3 = ORGate3::default();
        or_gate_3.input(&vec![a, b, c]);
        or_gate_3.update_state();
        assert_eq!(or_gate_3.output(), vec![d]);
    }

    #[test]
    fn test_and_gate_n_3_default() {
        let and_gate = ANDGateN::new(3);
        assert_eq!(and_gate.output(), vec![false]);
    }

    #[rstest]
    #[case(true, true, true, true)]
    #[case(true, true, false, false)]
    #[case(true, false, true, false)]
    #[case(true, false, false, false)]
    #[case(false, true, true, false)]
    #[case(false, true, false, false)]
    #[case(false, false, true, false)]
    #[case(false, false, false, false)]
    fn test_and_gate_n_3_with_truth_table(
        #[case] a: bool,
        #[case] b: bool,
        #[case] c: bool,
        #[case] d: bool,
    ) {
        let mut and_gate_3 = ANDGateN::new(3);
        and_gate_3.input(&vec![a, b, c]);
        and_gate_3.update_state();
        assert_eq!(and_gate_3.output(), vec![d]);
    }
}
