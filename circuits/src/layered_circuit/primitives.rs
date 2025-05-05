//! Holds primitives and core types used across the layered circuit

/// Enum declaring the gate ops this circuit works with
#[derive(Debug, Clone)]
pub enum GateOp {
    /// The addtion ops
    Add,
    /// The mul ops
    Mul,
}

/// This is the lowest unit of a layered circuit
#[derive(Debug, Clone)]
pub struct Gate {
    /// this op on this gate
    pub op: GateOp,
    /// This represents the inputs to the gate
    pub inputs: [usize; 2],
}

/// Layer of a the layered circuit
#[derive(Debug, Clone)]
pub struct Layer {
    /// This circuit layer is just a row of gates
    pub gates: Vec<Gate>,
}

/// This is the excecution trace of a circuit
#[derive(Debug, Clone)]
pub struct Evaluation<F> {
    /// The resulting evaluation for every layer
    pub layers: Vec<Vec<F>>,
}

impl Gate {
    pub fn new(op: GateOp, inputs: [usize; 2]) -> Self {
        Self { op, inputs }
    }
}

impl Layer {
    pub fn new(gates: Vec<Gate>) -> Self {
        Self { gates }
    }
}

impl<F> Evaluation<F> {
    pub fn new(layers: Vec<Vec<F>>) -> Self {
        Self { layers }
    }
}
