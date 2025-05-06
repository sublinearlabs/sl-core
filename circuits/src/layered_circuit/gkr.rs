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

        let mle_num_var = compute_num_vars(layer_index);
        let add_mle = mle_vec_to_poly(&add_usize_vec, mle_num_var);
        let mul_mle = mle_vec_to_poly(&add_usize_vec, mle_num_var);

        (add_mle, mul_mle)
    }
}
