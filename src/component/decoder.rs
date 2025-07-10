use crate::{
    circuit::{ANDGate, NOTGate, Potential, Wire},
    component::Component,
};

#[derive(Debug, Default, Clone)]
struct Decoder1_2 {
    input: [Wire; 1],
    output: [Wire; 2],
    not_gate: NOTGate,
}

impl Component for Decoder1_2 {
    fn get_pin_output(&self, position: usize) -> Potential {
        assert!(
            position < self.get_pin_count().1,
            "position must be less than {}",
            self.get_pin_count().1
        );
        self.output[position].output()
    }

    fn set_pin_input(&mut self, position: usize, value: &Potential) {
        assert!(
            position < self.get_pin_count().0,
            "position must be less than {}",
            self.get_pin_count().0
        );
        self.input[position].input(value);
    }

    fn update_state(&mut self) {
        self.not_gate.input(&self.input[0].output());
        self.output[0].input(&self.not_gate.output());
        self.output[1].input(&self.input[0].output());
    }

    fn get_pin_count(&self) -> (usize, usize) {
        (1, 2)
    }
}

#[derive(Debug, Default, Clone)]
struct Decoder2_4 {
    input: [Wire; 2],
    output: [Wire; 4],
    not_gate: [NOTGate; 2],
    and_gate: [ANDGate; 4],
}

impl Component for Decoder2_4 {
    fn get_pin_output(&self, position: usize) -> Potential {
        assert!(
            position < self.get_pin_count().1,
            "position must be less than {}",
            self.get_pin_count().1
        );
        self.output[position].output()
    }

    fn set_pin_input(&mut self, position: usize, value: &Potential) {
        assert!(
            position < self.get_pin_count().0,
            "position must be less than {}",
            self.get_pin_count().0
        );
        self.input[position].input(value);
    }

    fn update_state(&mut self) {
        self.not_gate[0].input(&self.input[0].output());
        self.not_gate[1].input(&self.input[1].output());
        self.and_gate[0].input(&self.not_gate[1].output(), &self.not_gate[0].output());
        self.and_gate[1].input(&self.not_gate[1].output(), &self.input[0].output());
        self.and_gate[2].input(&self.input[1].output(), &self.not_gate[0].output());
        self.and_gate[3].input(&self.input[1].output(), &self.input[0].output());
        self.output[0].input(&self.and_gate[0].output());
        self.output[1].input(&self.and_gate[1].output());
        self.output[2].input(&self.and_gate[2].output());
        self.output[3].input(&self.and_gate[3].output());
    }

    fn get_pin_count(&self) -> (usize, usize) {
        (2, 4)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        circuit::Potentials,
        component::{adder, decoder},
    };

    use super::*;
    use rstest::rstest;

    #[test]
    fn test_decoder1_2_default() {
        let decoder = Decoder1_2::default();
        assert_eq!(decoder.output(), vec![false, false]);
    }

    #[rstest]
    #[case(vec![true], vec![false, true])]
    #[case(vec![false], vec![true, false])]
    fn test_decoder1_2_with_truth_table(
        #[case] input: Vec<Potential>,
        #[case] output: Vec<Potential>,
    ) {
        let mut decoder = Decoder1_2::default();
        decoder.input(&input);
        assert_eq!(decoder.output(), output);
    }

    #[test]
    fn test_decoder2_4_default() {
        let decoder = Decoder2_4::default();
        assert_eq!(decoder.output(), vec![false, false, false, false]);
    }

    #[rstest]
    #[case(vec![false,false], vec![true, false,false,false])]
    #[case(vec![true,false], vec![false, true,false,false])]
    #[case(vec![false,true], vec![false, false,true,false])]
    #[case(vec![true,true], vec![false, false,false,true])]
    fn test_decoder2_4_with_truth_table(
        #[case] input: Vec<Potential>,
        #[case] output: Vec<Potential>,
    ) {
        let mut decoder = Decoder2_4::default();
        decoder.input(&input);
        assert_eq!(decoder.output(), output);
    }
}
