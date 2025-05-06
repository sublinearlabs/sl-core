//! Layer circuit extension for the libra version of the GKR protocol
use p3_field::{ExtensionField, Field};

use super::LayeredCircuit;
use crate::{interface::LibraGKRLayeredCircuitTr, layered_circuit::primitives::GateOp};

impl<F, E> LibraGKRLayeredCircuitTr<F, E> for LayeredCircuit
where
    F: Field + Copy + std::ops::Add<Output = F> + std::ops::Mul<Output = F>,
    E: ExtensionField<F>,
{
    type AddAndMulMLE = (Vec<(usize, usize, usize)>, Vec<(usize, usize, usize)>);

    fn add_and_mul_mle(&self, layer_index: usize) -> Self::AddAndMulMLE {
        assert!(
            layer_index < self.layers.len(),
            "Layer index is out of bounds"
        );

        let mut add_mle = Vec::new();
        let mut mul_mle = Vec::new();

        for (i, gate) in self.layers[layer_index].gates.iter().enumerate() {
            match gate.op {
                GateOp::Add => {
                    add_mle.push((i, gate.inputs[0], gate.inputs[1]));
                }
                GateOp::Mul => {
                    mul_mle.push((i, gate.inputs[0], gate.inputs[1]));
                }
            }
        }

        (add_mle, mul_mle)
    }
}

#[cfg(test)]
mod tests {
    use p3_field::{AbstractField, extension::BinomialExtensionField};
    use p3_goldilocks::Goldilocks;

    use crate::{
        interface::{CircuitTr, LibraGKRLayeredCircuitTr},
        layered_circuit::{
            LayeredCircuit,
            primitives::{Gate, GateOp, Layer},
        },
    };

    #[test]
    fn test_add_mul_mle_libra() {
        let circuit = LayeredCircuit::new(vec![
            Layer::new(vec![
                Gate::new(GateOp::Mul, [0, 1]),
                Gate::new(GateOp::Add, [2, 3]),
                Gate::new(GateOp::Add, [4, 5]),
                Gate::new(GateOp::Mul, [6, 7]),
            ]),
            Layer::new(vec![
                Gate::new(GateOp::Mul, [0, 1]),
                Gate::new(GateOp::Add, [2, 3]),
            ]),
            Layer::new(vec![Gate::new(GateOp::Mul, [0, 1])]),
        ]);

        let input = [1, 2, 3, 2, 1, 2, 4, 1]
            .into_iter()
            .map(Goldilocks::from_canonical_usize)
            .collect::<Vec<Goldilocks>>();
        let output = circuit.excecute(&input);
        assert_eq!(output.layers[3], vec![Goldilocks::from_canonical_usize(70)]);

        let (add_mle, mul_mle) = <LayeredCircuit as LibraGKRLayeredCircuitTr<
            Goldilocks,
            BinomialExtensionField<Goldilocks, 2>,
        >>::add_and_mul_mle(&circuit, 0);

        assert_eq!(add_mle, vec![(1, 2, 3), (2, 4, 5)]);
        assert_eq!(mul_mle, vec![(0, 0, 1), (3, 6, 7)]);
    }
}
