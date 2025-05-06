// //! Layer circuit extension for the libra version of the GKR protocol
// use p3_field::{ExtensionField, Field};
// use poly::mle::MultilinearPoly;

// use super::LayeredCircuit;
// use crate::{
//     interface::LibraGKRLayeredCircuitTr,
//     layered_circuit::{
//         primitives::GateOp,
//         utils::{compute_num_vars, get_gate_properties, mle_vec_to_poly},
//     },
// };

// impl<F, E> LibraGKRLayeredCircuitTr<F, E> for LayeredCircuit
// where
//     F: Field + Copy + std::ops::Add<Output = F> + std::ops::Mul<Output = F>,
//     E: ExtensionField<F>,
// {
//     type AddAndMulMLE = ();

//     fn add_and_mul_mle(&self, layer_index: usize) -> Self::AddAndMulMLE {
//         todo!()
//     }
// }
