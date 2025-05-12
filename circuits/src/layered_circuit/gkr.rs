//! Hold circuit extension for the GKR protocol
use p3_field::{ExtensionField, Field};
use poly::mle::MultilinearPoly;

use super::LayeredCircuit;
use crate::{
    interface::GKRLayeredCircuitTr,
    layered_circuit::{
        primitives::GateOp,
        utils::{compute_num_vars, get_gate_properties, mle_vec_to_poly},
    },
};

impl<F, E> GKRLayeredCircuitTr<F, E> for LayeredCircuit
where
    F: Field + Copy + std::ops::Add<Output = F> + std::ops::Mul<Output = F>,
    E: ExtensionField<F>,
{
    type AddAndMulMLE = (MultilinearPoly<F, E>, MultilinearPoly<F, E>);

    fn add_and_mul_mle(&self, layer_index: usize) -> Self::AddAndMulMLE {
        assert!(
            layer_index < self.layers.len(),
            "Layer index is out of bounds"
        );

        let mut add_usize_vec = Vec::new();
        let mut mul_usize_vec = Vec::new();

        for (i, gate) in self.layers[layer_index].gates.iter().enumerate() {
            match gate.op {
                GateOp::Add => {
                    let gate_props =
                        get_gate_properties(i, gate.inputs[0], gate.inputs[1], layer_index);
                    add_usize_vec.push(gate_props);
                }
                GateOp::Mul => {
                    let gate_props =
                        get_gate_properties(i, gate.inputs[0], gate.inputs[1], layer_index);
                    mul_usize_vec.push(gate_props);
                }
            }
        }

        let mle_num_var = compute_num_vars(layer_index, self.layers.len() - 1);

        let add_mle = mle_vec_to_poly(&add_usize_vec, mle_num_var);
        let mul_mle = mle_vec_to_poly(&mul_usize_vec, mle_num_var);

        (add_mle, mul_mle)
    }
}

#[cfg(test)]
mod test {
    use p3_field::{AbstractField, extension::BinomialExtensionField};
    use p3_goldilocks::Goldilocks as F;
    use poly::{Fields, MultilinearExtension};
    type E = BinomialExtensionField<F, 2>;
    use super::*;
    use crate::layered_circuit::primitives::{Gate, Layer};

    #[test]
    fn test_add_and_mul_mle() {
        //RECALL: this tree is kinda upside-down
        let layer_1 = Layer::new(vec![
            Gate::new(GateOp::Add, [0, 1]),
            Gate::new(GateOp::Mul, [2, 3]),
            Gate::new(GateOp::Mul, [4, 5]),
            Gate::new(GateOp::Mul, [6, 7]),
        ]);
        let layer_2 = Layer::new(vec![
            Gate::new(GateOp::Add, [0, 1]),
            Gate::new(GateOp::Mul, [2, 3]),
        ]);

        let layer_3 = Layer::new(vec![Gate::new(GateOp::Add, [0, 1])]);

        let circuit = LayeredCircuit::new(vec![layer_1, layer_2, layer_3]);

        let (add_mle, mul_mle) = <LayeredCircuit as GKRLayeredCircuitTr<
            F,
            BinomialExtensionField<F, 2>,
        >>::add_and_mul_mle(&circuit, circuit.layers.len() - 1);

        let is_zero = mul_mle
            .evaluations
            .iter()
            .all(|x| *x == Fields::Base(F::from_canonical_u32(0)));
        assert!(is_zero);
        let is_zero = add_mle
            .evaluations
            .iter()
            .all(|x| *x == Fields::Base(F::from_canonical_u32(0)));
        assert!(!is_zero);

        assert_eq!(
            add_mle.evaluate(&[
                Fields::Base(F::from_canonical_u32(0)),
                Fields::Base(F::from_canonical_u32(0)),
                Fields::Base(F::from_canonical_u32(1))
            ]),
            Fields::Extension(E::from_canonical_u32(1))
        );

        assert_eq!(
            add_mle.evaluate(&[
                Fields::Base(F::from_canonical_u32(0)),
                Fields::Base(F::from_canonical_u32(0)),
                Fields::Base(F::from_canonical_u32(0))
            ]),
            Fields::Extension(E::from_canonical_u32(0))
        );

        assert_eq!(
            add_mle.evaluate(&[
                Fields::Base(F::from_canonical_u32(1)),
                Fields::Base(F::from_canonical_u32(0)),
                Fields::Base(F::from_canonical_u32(0))
            ]),
            Fields::Extension(E::from_canonical_u32(0))
        );
        assert_eq!(
            add_mle.evaluate(&[
                Fields::Base(F::from_canonical_u32(1)),
                Fields::Base(F::from_canonical_u32(0)),
                Fields::Base(F::from_canonical_u32(1))
            ]),
            Fields::Extension(E::from_canonical_u32(0))
        );

        assert_eq!(
            add_mle.evaluate(&[
                Fields::Base(F::from_canonical_u32(1)),
                Fields::Base(F::from_canonical_u32(1)),
                Fields::Base(F::from_canonical_u32(1))
            ]),
            Fields::Extension(E::from_canonical_u32(0))
        );
    }

    #[test]
    fn test_add_and_mul_mle_layer_1() {
        // Create layers from bottom up (with layer 1 being the input layer)
        let layer_1 = Layer::new(vec![
            Gate::new(GateOp::Add, [0, 1]),
            Gate::new(GateOp::Mul, [2, 3]),
            Gate::new(GateOp::Mul, [4, 5]),
            Gate::new(GateOp::Mul, [6, 7]),
        ]);
        let layer_2 = Layer::new(vec![
            Gate::new(GateOp::Add, [0, 1]),
            Gate::new(GateOp::Mul, [2, 3]),
        ]);
        let layer_3 = Layer::new(vec![Gate::new(GateOp::Add, [0, 1])]);

        // Create circuit (the layers are in reverse order in your implementation)
        let circuit = LayeredCircuit::new(vec![layer_1, layer_2, layer_3]);

        // Test get_add_n_mul_mle for layer 2 (the middle layer)
        let layer_index = 1; // Middle layer (layer_2)

        let (add_mle, mul_mle) = <LayeredCircuit as GKRLayeredCircuitTr<
            F,
            BinomialExtensionField<F, 2>,
        >>::add_and_mul_mle(&circuit, layer_index);

        // There is one mul gate in layer 1, the mul mle should be non-zero
        assert!(
            mul_mle
                .evaluations
                .iter()
                .any(|&x| x != Fields::Base(F::from_canonical_u32(0)))
        );

        // There is one add gate in layer 1, the add mle should be non-zero
        assert!(
            add_mle
                .evaluations
                .iter()
                .any(|&x| x != Fields::Base(F::from_canonical_u32(0)))
        );

        // This should be the number of variables for the MLE (2 + 2 + 1 = 5 for this case)
        // The number of variables is likely log2(gates in current layer) + log2(gates in next layer) + log2(gates in previous layer)
        let expected_num_vars = 5;
        assert_eq!(add_mle.num_vars(), expected_num_vars);
        assert_eq!(mul_mle.num_vars(), expected_num_vars);

        // Evaluating the add mle at the correct binary combination should give a one
        // For the add gate [0,0,0,0,1]
        assert_eq!(
            add_mle.evaluate(&[
                Fields::Base(F::from_canonical_u32(0)),
                Fields::Base(F::from_canonical_u32(0)),
                Fields::Base(F::from_canonical_u32(0)),
                Fields::Base(F::from_canonical_u32(0)),
                Fields::Base(F::from_canonical_u32(1))
            ]),
            Fields::Extension(E::from_canonical_u32(1))
        );

        // Evaluating the mul mle at the correct binary combination should give a one
        // For the mul gate [1,1,0,1,1]
        assert_eq!(
            mul_mle.evaluate(&[
                Fields::Base(F::from_canonical_u32(1)),
                Fields::Base(F::from_canonical_u32(1)),
                Fields::Base(F::from_canonical_u32(0)),
                Fields::Base(F::from_canonical_u32(1)),
                Fields::Base(F::from_canonical_u32(1))
            ]),
            Fields::Extension(E::from_canonical_u32(1))
        );

        // Testing evaluations that should give zero for mul_mle
        let test_vectors_mul = vec![
            vec![1, 0, 0, 1, 1],
            vec![0, 1, 0, 1, 0],
            vec![0, 0, 0, 1, 1],
            vec![1, 1, 1, 1, 1],
        ];

        for test_vec in test_vectors_mul {
            let field_vec = test_vec
                .iter()
                .map(|&x| Fields::Base(F::from_canonical_u32(x)))
                .collect::<Vec<_>>();

            assert_eq!(
                mul_mle.evaluate(&field_vec),
                Fields::Extension(E::from_canonical_u32(0))
            );
        }

        // Testing evaluations that should give zero for add_mle
        let test_vectors_add = vec![
            vec![1, 0, 0, 1, 1],
            vec![0, 1, 0, 1, 0],
            vec![0, 0, 0, 1, 1],
            vec![1, 1, 1, 1, 1],
        ];

        for test_vec in test_vectors_add {
            let field_vec = test_vec
                .iter()
                .map(|&x| Fields::Base(F::from_canonical_u32(x)))
                .collect::<Vec<_>>();

            assert_eq!(
                add_mle.evaluate(&field_vec),
                Fields::Extension(E::from_canonical_u32(0))
            );
        }
    }
}
