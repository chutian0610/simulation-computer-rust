//!
//! Circuit module.
//!
//! This module defines the circuit and the components of the circuit.
//!
//! # Examples
//!
//! ```
//! use simulation_computer_rust::circuit::{ANDGate, ORGate, NOTGate, Wire};
//!
//! let mut and_gate = ANDGate::default();
//! let mut or_gate = ORGate::default();
//! let mut not_gate = NOTGate::default();
//! let mut wire = Wire::default();
//!
//! and_gate.input(&true, &false);
//! or_gate.input(&true, &false);
//! not_gate.input(&true);
//!
//! wire.input(&and_gate.output());
//!
//! assert_eq!(wire.output(), false);
//! assert_eq!(and_gate.output(), false);
//! assert_eq!(or_gate.output(), true);
//! assert_eq!(not_gate.output(), false);
//! ```

/// Potential in circuit.
pub type Potential = bool;

/// Wire in circuit.
#[derive(Debug, Default, Clone, Copy)]
pub struct Wire {
    potential: Potential,
}

impl Wire {
    /// Create a new wire.
    pub fn new(potential: Potential) -> Self {
        Self { potential }
    }
    /// Get the output of the wire.
    pub fn output(&self) -> Potential {
        self.potential
    }
    /// Set the input of the wire.
    pub fn input(&mut self, potential: &Potential) {
        self.potential = potential.to_owned();
    }
}

/// Potentials in circuit.

#[derive(Debug, Clone)]
pub struct Potentials {
    data: Vec<Potential>,
    little_endian: bool,
}

impl Potentials {
    /// Create a new Potentials.
    pub fn of_little_endian(potentials: Vec<Potential>) -> Self {
        Self {
            data: potentials,
            little_endian: true,
        }
    }

    pub fn get_data(&self, little_endian: bool) -> Vec<Potential> {
        if self.little_endian ^ little_endian {
            self.data.iter().rev().cloned().collect()
        }else {
            self.data.iter().cloned().collect()
        }
    }

    pub fn of_big_endian(potentials: Vec<Potential>) -> Self {
        Self {
            data: potentials,
            little_endian: false,
        }
    }

    /// Create a new Potentials from little endian string.
    /// 
    /// # Arguments
    ///
    /// * `little_endian` - The little endian string.
    /// * `ignore_padding` - Whether to ignore the padding.
    ///
    /// # Returns
    ///
    /// * `Self` - The new Potentials.
    pub fn from_little_endian(little_endian: &str, ignore_padding : bool) ->Self {
        let mut data = Vec::new();
        let mut ignore = true;
        for c in little_endian.chars().rev() {
            match c {
                '0' => {
                    ignore = ignore && true;
                    if !ignore_padding || (ignore_padding && !ignore) {
                        data.push(false);
                    } 
                },
                '1' => {
                    ignore = ignore && false;
                    data.push(true);
                } 
                ' ' => continue,
                _ => panic!("Invalid character in little endian string"),
            }
        }
        data.reverse();
        Self {
            data,
            little_endian: true,
        }
    }

    /// Create a new Potentials from big endian string.
    /// 
    /// # Arguments
    ///
    /// * `big_endian` - The big endian string.
    /// * `ignore_padding` - Whether to ignore the padding.
    ///
    /// # Returns
    ///
    /// * `Self` - The new Potentials.
    pub fn from_big_endian(big_endian: &str, ignore_padding : bool) ->Self {
        let mut data = Vec::new();
        let mut ignore = true;
        for c in big_endian.chars() {
            match c {
                '0' => {
                    ignore = ignore && true;
                    if !ignore_padding {
                        data.push(false);
                    } else {
                        if !ignore {
                            data.push(false);
                        } 
                    } 

                },
                '1' => {
                    ignore = ignore && false;
                    data.push(true);
                } 
                ' ' => continue,
                _ => panic!("Invalid character in little endian string"),
            }
        }
        Self {
            data,
            little_endian: false,
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Get the raw data of the Potentials.
    ///
    /// # Arguments
    ///
    /// * `little_endian` - The endian of the Potentials.
    /// * `format_type` - The format type of the Potentials.
    ///     * `0` - No format.
    ///     * `1` - 4 bits per group(Nibble).
    ///     * `2` - 8 bits per group(Byte).
    ///
    /// # Returns
    ///
    /// * `String` - The raw data of the Potentials.
    pub fn to_raw(&self, little_endian: bool, format_type: usize) -> String {
        assert!(format_type <= 2);
        fn format(items: Vec<&Potential>, format_type: usize, little_endian: bool) -> String {
            let mut s = String::with_capacity(items.len());
            let length = items.len();
            let padding: usize = if format_type == 1 && length %4 !=0 {
                4-(length%4)
            } else if format_type == 2 && length %8 !=0 {
                 8-(length%8)
            } else {0};
            let mut cursor = 0;
            if !little_endian {
                // big endian padding at the beginning
                if padding >0 {
                    for _i in 0..padding  {
                        s.push('0');               
                    }
                }
                cursor += padding;
            }
            for p in items {
                s.push(if *p { '1' } else { '0' });
                cursor += 1;
                if format_type == 1 && cursor % 4 == 0 && cursor != 0 {
                    s.push(' ');
                } else if format_type == 2 && cursor % 8 == 0 && cursor != 0 {
                    s.push(' ');
                }
            }
            if little_endian {
                // little endian padding at the end
                if padding >0 {
                    for _i in 0..padding  {
                        s.push('0');               
                    }
                }
            }
            // may end With ''
            s.trim_end().to_owned()
        }
        if self.little_endian ^ little_endian {
            // target endian different to current endian
            let items: Vec<&Potential> = self.data.iter().rev().collect();
            format(items, format_type, little_endian)
        } else {
            // target endian same as current endian
            let items: Vec<&Potential> = self.data.iter().collect();
            format(items, format_type, little_endian)
        }
    }

    /// Get the little endian raw data of the Potentials.
    ///
    /// # Arguments
    ///
    /// * `format_type` - The format type of the Potentials. Default `1`
    ///     * `0` - No format.
    ///     * `1` - 4 bits per group(Nibble).
    ///     * `2` - 8 bits per group(Byte).
    ///
    /// # Returns
    ///
    /// * `String` - The little endian raw data of the Potentials.
    pub fn to_little_endian(&self, format_type: Option<usize>) -> String {
        self.to_raw(true, format_type.unwrap_or(1))
    }

    /// Get the big endian raw data of the Potentials.
    ///
    /// # Arguments
    ///
    /// * `format_type` - The format type of the Potentials.Default `1`
    ///     * `0` - No format.
    ///     * `1` - 4 bits per group(Nibble).
    ///     * `2` - 8 bits per group(Byte).
    ///
    /// # Returns
    ///
    /// * `String` - The big endian raw data of the Potentials.
    pub fn to_big_endian(&self, format_type: Option<usize>) -> String {
        self.to_raw(false, format_type.unwrap_or(1))
    }
}

/// Operator not in circuit.
pub fn operator_not(a: &Potential) -> Potential {
    !a
}

/// Operator and in circuit.
pub fn operator_and(a: &Potential, b: &Potential) -> Potential {
    a.to_owned() && b.to_owned()
}

/// Operator or in circuit.
pub fn operator_or(a: &Potential, b: &Potential) -> Potential {
    a.to_owned() || b.to_owned()
}
/// Operator xor in circuit.
pub fn operator_xor(a: &Potential, b: &Potential) -> Potential {
    a ^ b
}
/// Operator nand in circuit.
pub fn operator_nand(a: &Potential, b: &Potential) -> Potential {
    operator_not(&operator_and(a, b))
}

/// Operator nor in circuit.
pub fn operator_nor(a: &Potential, b: &Potential) -> Potential {
    operator_not(&operator_or(a, b))
}

/// AND gate in circuit.
#[derive(Debug, Default, Clone)]
pub struct ANDGate {
    wire: Wire,
}
impl ANDGate {
    /// Get the output of the gate.
    pub fn output(&self) -> Potential {
        self.wire.output()
    }
    /// Set the input of the gate.
    pub fn input(&mut self, a: &Potential, b: &Potential) {
        self.wire.input(&operator_and(a, b));
    }
}

/// OR gate in circuit.
#[derive(Debug, Default, Clone)]
pub struct ORGate {
    wire: Wire,
}
impl ORGate {
    /// Get the output of the gate.
    pub fn output(&self) -> Potential {
        self.wire.output()
    }
    /// Set the input of the gate.
    pub fn input(&mut self, a: &Potential, b: &Potential) {
        self.wire.input(&operator_or(a, b));
    }
}

/// NOT gate in circuit.
#[derive(Debug, Default, Clone)]
pub struct NOTGate {
    wire: Wire,
}
impl NOTGate {
    /// Get the output of the gate.
    pub fn output(&self) -> Potential {
        self.wire.output()
    }

    /// Set the input of the gate.
    pub fn input(&mut self, a: &Potential) {
        self.wire.input(&operator_not(a));
    }
}

/// XOR gate in circuit.
#[derive(Debug, Default, Clone)]
pub struct XORGate {
    wire: Wire,
}
impl XORGate {
    /// Get the output of the gate.
    pub fn output(&self) -> Potential {
        self.wire.output()
    }

    /// Set the input of the gate.
    pub fn input(&mut self, a: &Potential, b: &Potential) {
        self.wire.input(&operator_xor(a, b));
    }
}
/// NAND gate in circuit.
#[derive(Debug, Default, Clone)]
pub struct NANDGate {
    wire: Wire,
}
impl NANDGate {
    /// Get the output of the gate.
    pub fn output(&self) -> Potential {
        self.wire.output()
    }
    /// Set the input of the gate.
    pub fn input(&mut self, a: &Potential, b: &Potential) {
        self.wire.input(&operator_nand(a, b));
    }
}

/// NOR gate in circuit.
#[derive(Debug, Default, Clone)]
pub struct NORGate {
    wire: Wire,
}
impl NORGate {
    /// Get the output of the gate.
    pub fn output(&self) -> Potential {
        self.wire.output()
    }
    /// Set the input of the gate.
    pub fn input(&mut self, a: &Potential, b: &Potential) {
        self.wire.input(&operator_nor(a, b));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    #[test]
    fn test_wire() {
        let mut wire = Wire::default();
        assert_eq!(wire.output(), false);
        wire.input(&true);
        assert_eq!(wire.output(), true);
        wire.input(&false);
        assert_eq!(wire.output(), false);
    }

    #[test]
    fn test_wire_copy() {
        let mut wire1: Wire = Wire::default();
        assert_eq!(wire1.output(), false);
        // println!("地址1: {:p}", &wire1);
        let wire2 = wire1;
        assert_eq!(wire2.output(), false);
        // println!("地址2: {:p}", &wire2);
        wire1.input(&true);
        // println!("地址1: {:p}", &wire1);
        assert_eq!(wire1.output(), true);
        assert_eq!(wire2.output(), false);
    }

    #[test]
    fn test_not_gate_default() {
        let not_gate = NOTGate::default();
        assert_eq!(not_gate.output(), false);
    }

    #[test]
    fn test_and_gate_default() {
        let and_gate = ANDGate::default();
        assert_eq!(and_gate.output(), false);
    }

    #[test]
    fn test_or_gate_default() {
        let or_gate = ORGate::default();
        assert_eq!(or_gate.output(), false);
    }

    #[test]
    fn test_xor_gate_default() {
        let xor_gate = XORGate::default();
        assert_eq!(xor_gate.output(), false);
    }

    #[rstest]
    #[case(true, true, true)]
    #[case(true, false, false)]
    #[case(false, true, false)]
    #[case(false, false, false)]
    fn test_and_gate_with_truth_table(#[case] a: bool, #[case] b: bool, #[case] c: bool) {
        let mut and_gate = ANDGate::default();
        and_gate.input(&a, &b);
        assert_eq!(and_gate.output(), c);
    }

    #[rstest]
    #[case(true, true, true)]
    #[case(true, false, true)]
    #[case(false, true, true)]
    #[case(false, false, false)]
    fn test_or_gate_with_truth_table(#[case] a: bool, #[case] b: bool, #[case] c: bool) {
        let mut or_gate = ORGate::default();
        or_gate.input(&a, &b);
        assert_eq!(or_gate.output(), c);
    }

    #[rstest]
    #[case(true, false)]
    #[case(false, true)]
    fn test_not_gate_with_truth_table(#[case] a: bool, #[case] c: bool) {
        let mut not_gate = NOTGate::default();
        not_gate.input(&a);
        assert_eq!(not_gate.output(), c);
    }

    #[rstest]
    #[case(true, true, false)]
    #[case(true, false, true)]
    #[case(false, true, true)]
    #[case(false, false, false)]
    fn test_xor_gate_with_truth_table(#[case] a: bool, #[case] b: bool, #[case] c: bool) {
        let mut xor_gate = XORGate::default();
        xor_gate.input(&a, &b);
        assert_eq!(xor_gate.output(), c);
    }

    #[test]
    fn test_nand_gate_default() {
        let nand_gate = NANDGate::default();
        assert_eq!(nand_gate.output(), false);
    }

    #[rstest]
    #[case(true, true, false)]
    #[case(true, false, true)]
    #[case(false, true, true)]
    #[case(false, false, true)]
    fn test_nand_gate_with_truth_table(#[case] a: bool, #[case] b: bool, #[case] c: bool) {
        let mut nand_gate = NANDGate::default();
        nand_gate.input(&a, &b);
        assert_eq!(nand_gate.output(), c);
    }

    #[test]
    fn test_nor_gate_default() {
        let nor_gate = NORGate::default();
        assert_eq!(nor_gate.output(), false);
    }

    #[rstest]
    #[case(true, true, false)]
    #[case(true, false, false)]
    #[case(false, true, false)]
    #[case(false, false, true)]
    fn test_nor_gate_with_truth_table(#[case] a: bool, #[case] b: bool, #[case] c: bool) {
        let mut nor_gate = NORGate::default();
        nor_gate.input(&a, &b);
        assert_eq!(nor_gate.output(), c);
    }

    #[rstest]
    #[case(vec![true,true,true], "111")]
    #[case(vec![true,true,false], "110")]
    #[case(vec![true,false,true], "101")]
    #[case(vec![true,false,false], "100")]
    #[case(vec![false,true,true], "011")]
    #[case(vec![false,true,false], "010")]
    #[case(vec![false,false,true], "001")]
    #[case(vec![false,false,false], "000")]
    fn test_potentials_little_endian_2_little(#[case] data: Vec<Potential>,#[case] raw:String) {
        let potentials: Potentials = Potentials::of_little_endian(data);
        assert_eq!(potentials.to_little_endian(Some(0)),raw);
    }
    #[rstest]
    #[case(vec![true,true,true],"111")]
    #[case(vec![true,true,false], "011")]
    #[case(vec![true,false,true], "101")]
    #[case(vec![true,false,false], "001")]
    #[case(vec![false,true,true], "110")]
    #[case(vec![false,true,false], "010")]
    #[case(vec![false,false,true], "100")]
    #[case(vec![false,false,false], "000")]
    fn test_potentials_big_endian_2_little(#[case] data: Vec<Potential>,#[case] raw:String) {
        let potentials: Potentials = Potentials::of_big_endian(data);
        assert_eq!(potentials.to_little_endian(Some(0)),raw);
    }
    #[rstest]
    #[case(vec![true,true,true,true,false,false], "1111 0000")]
    #[case(vec![true,true,true,true,false], "1111 0000")]
    #[case(vec![true,true,true,true,false,false,false,false], "1111 0000")]
    #[case(vec![true,true,true,true,false,false,false,false,false], "1111 0000 0000")]
    fn test_potentials_little_endian_format_4(#[case] data: Vec<Potential>,#[case] raw:String) {
        let potentials: Potentials = Potentials::of_little_endian(data);
        assert_eq!(potentials.to_little_endian(Some(1)),raw);
    }
    #[rstest]
    #[case(vec![true,true,true,true,false,false], "0011 1100")]
    #[case(vec![true,true,true,true,false], "0001 1110")]
    #[case(vec![true,true,true,true,false,false,false,false], "1111 0000")]
    #[case(vec![true,true,true,true,false,false,false,false,false], "0001 1110 0000")]
    fn test_potentials_big_endian_format_4(#[case] data: Vec<Potential>,#[case] raw:String) {
        let potentials: Potentials = Potentials::of_big_endian(data);
        assert_eq!(potentials.to_big_endian(Some(1)),raw);
    }

    #[rstest]
    #[case("1111 0000",vec![true,true,true,true,false,false,false,false])]
    #[case("1111 1100",vec![true,true,true,true,true,true,false,false])]
    fn test_potentials_from_little_endian_str_01(#[case] raw: String,#[case] data: Vec<Potential>) {
        let potentials: Potentials = Potentials::from_little_endian(&raw,false);
        assert_eq!(potentials.data,data);
    }
    #[rstest]
    #[case("0011 0000",vec![false,false,true,true])]
    #[case("0011 1100",vec![false,false,true,true,true,true])]
    fn test_potentials_from_little_endian_str_02(#[case] raw: String,#[case] data: Vec<Potential>) {
        let potentials: Potentials = Potentials::from_little_endian(&raw,true);
        assert_eq!(potentials.data,data);
    }
    #[rstest]
    #[case("0011 0000",vec![false,false,true,true,false,false,false,false])]
    #[case("0011 1100",vec![false,false,true,true,true,true,false,false])]
    fn test_potentials_from_big_endian_str_01(#[case] raw: String,#[case] data: Vec<Potential>) {
        let potentials: Potentials = Potentials::from_big_endian(&raw,false);
        assert_eq!(potentials.data,data);
    }
        #[rstest]
    #[case("0011 0000",vec![true,true,false,false,false,false])]
    #[case("0011 1100",vec![true,true,true,true,false,false])]
    fn test_potentials_from_big_endian_str_02(#[case] raw: String,#[case] data: Vec<Potential>) {
        let potentials: Potentials = Potentials::from_big_endian(&raw,true);
        assert_eq!(potentials.data,data);
    }
}
