use crate::circuit::{ANDGate, Wire};

use super::Component;

#[derive(Default)]
pub struct ANDGate3 {
    input: [Wire; 3],
    and_gate: [ANDGate; 2],
    output: Wire,
}

impl Component for ANDGate3 {
    fn get_pin_output(&self, position: usize) -> bool {
        assert!(
            position == self.get_pin_count().1,
            "position must  eq {}",
            self.get_pin_count().1
        );
        self.output.output()
    }

    fn set_pin_input(&mut self, position: usize, value: bool) {
        assert!(
            position < self.get_pin_count().0,
            "position must be less than {}",
            self.get_pin_count().0
        );
        self.input[position].input(value);
    }

    fn update_state(&mut self) {
        self.and_gate[0].input(self.input[0].output(), self.input[1].output());
        self.and_gate[1].input(self.and_gate[0].output(), self.input[2].output());
        self.output.input(self.and_gate[1].output());
    }

    fn get_pin_count(&self) -> (usize, usize) {
        (3, 1)
    }
}
