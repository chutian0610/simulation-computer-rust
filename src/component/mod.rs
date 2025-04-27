use std::vec;

use crate::circuit::Potential;

pub mod big_gates;
pub trait Component {
    /// Obtain the output of the pin at the corresponding position of the component
    fn get_pin_output(&self, position: usize) -> Potential;
    /// set the input of the pin at the corresponding position of the component
    fn set_pin_input(&mut self, position: usize, value: &Potential);
    /// update the state of the component
    fn update_state(&mut self);
    /// get the number of pins of the component
    fn get_pin_count(&self) -> (usize, usize);

    /// batch input of the component
    fn input(&mut self, vec: &Vec<Potential>) {
        assert!(vec.len() <= self.get_pin_count().0);
        for i in 0..vec.len() {
            self.set_pin_input(i, &vec[i]);
        }
    }
    /// batch ouput of the component
    fn output(&self) -> Vec<Potential> {
        let len: usize = self.get_pin_count().1;
        let mut vec: Vec<Potential> = vec![false; len];
        for i in 0..vec.len() {
            vec[i] = self.get_pin_output(i);
        }
        vec
    }
}
