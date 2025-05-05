//! Declaring interfaces for circuit and it related primitives

/// Interface discribing the circuit structure
pub trait CircuitTr {
    /// This is the Field the Circuit operates on
    type F;
    /// This is the resulting evaluation of the circuit
    type CircuitEvaluation;

    /// This function excecutes the Circuit with a Vec of inputs
    fn excecute(&self, input: &[Self::F]) -> Self::CircuitEvaluation;
}

/// An extension of the circuit with implemention to get layer circuit props
pub trait LayeredCircuitTr: CircuitTr {
    /// This is the type for the Add and Mul MLE (e.g. (MLE, MLE), (VPoly, VPoly))
    type AddAndMulMLE;

    /// This function returns the add and mul mle for the specified layer
    fn add_and_mul_mle(&self, layer_index: usize) -> Self::AddAndMulMLE;
}
