//! The Circuit execution functionality is implemented on the Layered circuit here.
use super::{
    LayeredCircuit,
    primitives::{Evaluation, GateOp},
};
use crate::interface::CircuitTr;

impl<F> CircuitTr<F> for LayeredCircuit
where
    F: Copy + std::ops::Add<Output = F> + std::ops::Mul<Output = F>,
{
    type CircuitEvaluation = Evaluation<F>;

    fn excecute(&self, input: &[F]) -> Self::CircuitEvaluation {
        let mut layers = vec![];

        let mut current_input = input;
        layers.push(input.to_vec());

        for layer in self.layers.iter() {
            let temp_layer = layer
                .gates
                .iter()
                .map(|e| match e.op {
                    GateOp::Add => current_input[e.inputs[0]] + current_input[e.inputs[1]],
                    GateOp::Mul => current_input[e.inputs[0]] * current_input[e.inputs[1]],
                })
                .collect();
            layers.push(temp_layer);
            current_input = &layers[layers.len() - 1];
        }

        Evaluation::new(layers)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layered_circuit::primitives::{Gate, Layer};
    use p3_field::{AbstractField, extension::BinomialExtensionField};
    use p3_goldilocks::Goldilocks as F;
    use poly::Fields;
    type E = BinomialExtensionField<F, 2>;

    fn generate_input(raw_input: &[u32], is_ext_field: bool) -> Vec<Fields<F, E>> {
        raw_input
            .iter()
            .map(|e| {
                if is_ext_field {
                    Fields::Extension(E::from_canonical_u32(*e))
                } else {
                    Fields::Base(F::from_canonical_u32(*e))
                }
            })
            .collect()
    }

    // Helper function to create a simple test circuit
    fn create_test_circuit() -> LayeredCircuit {
        // Create a 3-layer circuit:
        // Layer 1: 2 ADD gates, 2 MUL gate
        // Layer 2: 1 MUL gate, 1 Mul gate
        // Layer 3: 1 MUL
        let layer1 = Layer::new(vec![
            Gate::new(GateOp::Mul, [0, 1]),
            Gate::new(GateOp::Add, [2, 3]),
            Gate::new(GateOp::Add, [4, 5]),
            Gate::new(GateOp::Mul, [6, 7]),
        ]);
        let layer2 = Layer::new(vec![
            Gate::new(GateOp::Mul, [0, 1]),
            Gate::new(GateOp::Add, [2, 3]),
        ]);
        let layer3 = Layer::new(vec![Gate::new(GateOp::Mul, [0, 1])]);

        LayeredCircuit::new(vec![layer1, layer2, layer3])
    }

    #[test]
    fn test_circuit_exec_base() {
        let circuit = create_test_circuit();
        let input = generate_input(&vec![1, 2, 3, 2, 1, 2, 4, 1], false);
        let trace = circuit.excecute(&input);
        let out = &trace.layers[trace.layers.len() - 1];

        assert_eq!(out[0], Fields::Base(F::from_canonical_u32(70)))
    }

    #[test]
    fn test_circuit_exec_ext() {
        let circuit = create_test_circuit();
        let input = generate_input(&vec![1, 2, 3, 2, 1, 2, 4, 1], true);
        let trace = circuit.excecute(&input);
        let out = &trace.layers[trace.layers.len() - 1];

        assert_eq!(out[0], Fields::Extension(E::from_canonical_u32(70)))
    }
}
