pub mod big_gates;
pub trait Component {
    /// Obtain the output of the pin at the corresponding position of the component
    fn get_pin_output(&self, position: usize) -> bool;
    /// set the input of the pin at the corresponding position of the component
    fn set_pin_input(&mut self, position: usize, value: bool);
    /// update the state of the component
    fn update_state(&mut self);
    /// get the number of pins of the component
    fn get_pin_count(&self) -> (usize, usize);
}
