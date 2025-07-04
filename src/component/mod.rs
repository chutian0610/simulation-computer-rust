use std::vec;

use crate::circuit::Potential;

pub mod adder;
pub mod big_gates;

/// A trait representing a component with input and output pins.
pub trait Component {
    /// Obtain the output of the pin at the corresponding position of the component.
    ///
    /// # Arguments
    /// * `position` - The position of the pin.
    ///
    /// # Returns
    /// The potential value of the pin at the specified position.
    fn get_pin_output(&self, position: usize) -> Potential;

    /// Set the input of the pin at the corresponding position of the component.
    ///
    /// # Arguments
    /// * `position` - The position of the pin.
    /// * `value` - A reference to the potential value to set.
    fn set_pin_input(&mut self, position: usize, value: &Potential);

    /// Update the state of the component.
    fn update_state(&mut self);

    /// Get the number of input and output pins of the component.
    ///
    /// # Returns
    /// A tuple containing the number of input pins and output pins.
    fn get_pin_count(&self) -> (usize, usize);

    /// Perform batch input for the component.
    ///
    /// # Arguments
    /// * `vec` - A reference to a vector of potential values.
    fn input(&mut self, vec: &Vec<Potential>) {
        assert!(vec.len() <= self.get_pin_count().0);
        for i in 0..vec.len() {
            self.set_pin_input(i, &vec[i]);
        }
    }
    /// Perform batch input for the component, and update the state of the component.
    ///
    /// # Arguments
    /// * `vec` - A reference to a vector of potential values.
    fn fire(&mut self, vec: &Vec<Potential>) {
        self.input(vec);
        self.update_state();
    }

    /// Perform batch output for the component.
    ///
    /// # Returns
    /// A vector containing the potential values of all output pins.
    fn output(&self) -> Vec<Potential> {
        let len: usize = self.get_pin_count().1;
        let mut vec: Vec<Potential> = vec![false; len];
        for i in 0..vec.len() {
            vec[i] = self.get_pin_output(i);
        }
        vec
    }
}
