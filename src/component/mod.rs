trait Component {
    fn get_output(&self, index: i32) -> bool;
    fn set_input(&mut self, index: i32, value: bool) -> bool;
    fn update_state(&mut self) -> bool;
}
