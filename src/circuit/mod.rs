use std::ops::Deref;

/// Wire in circuit.
#[derive(Default)]
pub struct Wire {
    potential: bool,
}

impl Deref for Wire {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.potential
    }
}
impl Wire {
    /// Create a new wire.
    pub fn new(potential: bool) -> Self {
        Self { potential }
    }
    /// Get the output of the wire.
    pub fn output(&self) -> bool {
        self.potential
    }
    /// Set the input of the wire.
    pub fn input(&mut self, potential: bool) {
        self.potential = potential;
    }
}

/// Operator not in circuit.
pub const fn operator_not(a: bool) -> bool {
    !a
}

/// Operator and in circuit.
pub const fn operator_and(a: bool, b: bool) -> bool {
    a && b
}

/// Operator or in circuit.
pub const fn operator_or(a: bool, b: bool) -> bool {
    a || b
}
/// Operator xor in circuit.
pub const fn operator_xor(a: bool, b: bool) -> bool {
    a ^ b
}
/// Operator nand in circuit.
pub const fn operator_nand(a: bool, b: bool) -> bool {
    operator_not(operator_and(a, b))
}

/// Operator nor in circuit.
pub const fn operator_nor(a: bool, b: bool) -> bool {
    operator_not(operator_or(a, b))
}

/// AND gate in circuit.
#[derive(Default)]
pub struct ANDGate {
    wire: Wire,
}
impl ANDGate {
    /// Get the output of the gate.
    pub fn output(&self) -> bool {
        self.wire.output()
    }
    /// Set the input of the gate.
    pub fn input(&mut self, a: bool, b: bool) {
        self.wire.input(operator_and(a, b));
    }
}

/// OR gate in circuit.
#[derive(Default)]
pub struct ORGate {
    wire: Wire,
}
impl ORGate {
    /// Get the output of the gate.
    pub fn output(&self) -> bool {
        self.wire.output()
    }
    /// Set the input of the gate.
    pub fn input(&mut self, a: bool, b: bool) {
        self.wire.input(operator_or(a, b));
    }
}

/// NOT gate in circuit.
#[derive(Default)]
pub struct NOTGate {
    wire: Wire,
}
impl NOTGate {
    /// Get the output of the gate.
    pub fn output(&self) -> bool {
        self.wire.output()
    }

    /// Set the input of the gate.
    pub fn input(&mut self, a: bool) {
        self.wire.input(operator_not(a));
    }
}

/// XOR gate in circuit.
#[derive(Default)]
pub struct XORGate {
    wire: Wire,
}
impl XORGate {
    /// Get the output of the gate.
    pub fn output(&self) -> bool {
        self.wire.output()
    }

    /// Set the input of the gate.
    pub fn input(&mut self, a: bool, b: bool) {
        self.wire.input(operator_xor(a, b));
    }
}
/// NAND gate in circuit.
#[derive(Default)]
pub struct NANDGate {
    wire: Wire,
}
impl NANDGate {
    /// Get the output of the gate.
    pub fn output(&self) -> bool {
        self.wire.output()
    }
    /// Set the input of the gate.
    pub fn input(&mut self, a: bool, b: bool) {
        self.wire.input(operator_nand(a, b));
    }
}

/// NOR gate in circuit.
#[derive(Default)]
pub struct NORGate {
    wire: Wire,
}
impl NORGate {
    /// Get the output of the gate.
    pub fn output(&self) -> bool {
        self.wire.output()
    }
    /// Set the input of the gate.
    pub fn input(&mut self, a: bool, b: bool) {
        self.wire.input(operator_nor(a, b));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wire() {
        let mut wire = Wire::default();
        assert_eq!(wire.output(), false);
        wire.input(true);
        assert_eq!(wire.output(), true);
        wire.input(false);
        assert_eq!(wire.output(), false);
    }
    #[test]
    fn test_and_gate() {
        let mut and_gate = ANDGate::default();
        assert_eq!(and_gate.output(), false);

        and_gate.input(true, true);
        assert_eq!(and_gate.output(), true);

        and_gate.input(true, false);
        assert_eq!(and_gate.output(), false);

        and_gate.input(false, true);
        assert_eq!(and_gate.output(), false);

        and_gate.input(false, false);
        assert_eq!(and_gate.output(), false);
    }

    #[test]
    fn test_or_gate() {
        let mut or_gate = ORGate::default();
        assert_eq!(or_gate.output(), false);
        or_gate.input(true, true);
        assert_eq!(or_gate.output(), true);
        or_gate.input(true, false);
        assert_eq!(or_gate.output(), true);
        or_gate.input(false, true);
        assert_eq!(or_gate.output(), true);
        or_gate.input(false, false);
        assert_eq!(or_gate.output(), false);
    }

    #[test]
    fn test_not_gate() {
        let mut not_gate = NOTGate::default();
        assert_eq!(not_gate.output(), false);
        not_gate.input(true);
        assert_eq!(not_gate.output(), false);
        not_gate.input(false);
        assert_eq!(not_gate.output(), true);
    }

    #[test]
    fn test_xor_gate() {
        let mut xor_gate = XORGate::default();
        assert_eq!(xor_gate.output(), false);

        xor_gate.input(true, true);
        assert_eq!(xor_gate.output(), false);
        xor_gate.input(true, false);
        assert_eq!(xor_gate.output(), true);
        xor_gate.input(false, true);
        assert_eq!(xor_gate.output(), true);
        xor_gate.input(false, false);
        assert_eq!(xor_gate.output(), false);
    }

    #[test]
    fn test_nand_gate() {
        let mut nand_gate = NANDGate::default();
        assert_eq!(nand_gate.output(), false);

        nand_gate.input(true, true);
        assert_eq!(nand_gate.output(), false);
        nand_gate.input(true, false);
        assert_eq!(nand_gate.output(), true);
        nand_gate.input(false, true);
        assert_eq!(nand_gate.output(), true);
        nand_gate.input(false, false);
        assert_eq!(nand_gate.output(), true);
    }

    #[test]
    fn test_nor_gate() {
        let mut nor_gate = NORGate::default();
        assert_eq!(nor_gate.output(), false);

        nor_gate.input(true, true);
        assert_eq!(nor_gate.output(), false);
        nor_gate.input(true, false);
        assert_eq!(nor_gate.output(), false);
        nor_gate.input(false, true);
        assert_eq!(nor_gate.output(), false);
        nor_gate.input(false, false);
        assert_eq!(nor_gate.output(), true);
    }
}
