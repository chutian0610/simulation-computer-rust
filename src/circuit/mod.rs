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

    pub fn of_big_endian(potentials: Vec<Potential>) -> Self {
        Self {
            data: potentials,
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
        fn format(data: &Vec<Potential>, rev: bool, format_type: usize) -> String {
            let mut s = String::with_capacity(data.len());
            let items: Vec<&Potential> = if rev {
                data.iter().rev().collect()
            } else {
                data.iter().collect()
            };
            let mut count = 0;
            for p in items {
                s.push(if *p { '1' } else { '0' });
                if format_type == 1 && count % 4 == 0 && count != 0 {
                    s.push(' ');
                } else if format_type == 2 && count % 8 == 0 && count != 0 {
                    s.push(' ');
                }
                count += 1;
            }
            s
        }

        if self.little_endian ^ little_endian {
            // target endian different to current endian
            format(&self.data, true, format_type)
        } else {
            // target endian same as current endian
            format(&self.data, false, format_type)
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
}
