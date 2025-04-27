use crate::circuit::{ANDGate, Potential, Wire};

use super::Component;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_and_gate_3() {
        let mut and_gate_3 = ANDGate3::default();

        let vec = vec![true, true, true];
        and_gate_3.input(&vec);
        and_gate_3.update_state();
        assert_eq!(and_gate_3.output(), vec![true]);

        let vec = vec![true, false, true];
        and_gate_3.input(&vec);
        and_gate_3.update_state();
        assert_eq!(and_gate_3.output(), vec![false]);

        let vec = vec![false, false, false];
        and_gate_3.input(&vec);
        and_gate_3.update_state();
        assert_eq!(and_gate_3.output(), vec![false]);
    }
}
