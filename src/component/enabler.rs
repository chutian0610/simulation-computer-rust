use crate::{
    circuit::{ANDGate, Potential, Wire},
    component::Component,
};

/// a n-way enabler in circuit.
/// the input is n+1 bits, and the output is n bits.
///
/// ```ascii
///                i0  i1  i2  i3   
///                │   │   │   │    
///            ┌───┴───┴───┴───┴───┐
///            │                   │
/// switcher───┤      Enabler      │
///            │                   │
///            └───┬───┬───┬───┬───┘
///                │   │   │   │    
///                o0  o1  o2  o3   
/// ```
///
/// # input  
/// the first n bit is the input, and the last 1 bit is switcher
///
/// # output
/// if the switcher is high, the output is the input.
/// if the switcher is low, the output is low.
#[derive(Debug, Default, Clone)]
struct EnablerN {
    n_way: usize,
    input: Vec<Wire>,
    and_gates: Vec<ANDGate>,
    output: Vec<Wire>,
}

impl EnablerN {
    pub fn new(n_way: usize) -> Self {
        Self {
            n_way,
            input: vec![Wire::default(); n_way + 1],
            and_gates: vec![ANDGate::default(); n_way],
            output: vec![Wire::default(); n_way],
        }
    }
}

impl Component for EnablerN {
    fn get_pin_count(&self) -> (usize, usize) {
        (self.n_way + 1, self.n_way)
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
        let switcher = self.input[self.n_way].output();
        for i in 0..self.n_way {
            let and_gate = &mut self.and_gates[i];
            and_gate.input(&self.input[i].output(), &switcher);
            self.output[i].input(&and_gate.output());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enable_default() {
        let enabler = EnablerN::new(4);
        assert_eq!(enabler.output(), vec![false, false, false, false]);
    }
    #[test]
    fn test_enable_on() {
        let mut enabler = EnablerN::new(4);
        enabler.input(&vec![true, false, false, true, true]);
        assert_eq!(enabler.output(), vec![true, false, false, true]);
    }

    #[test]
    fn test_enable_off() {
        let mut enabler = EnablerN::new(4);
        enabler.input(&vec![true, false, false, true, false]);
        assert_eq!(enabler.output(), vec![false, false, false, false]);
    }
}
