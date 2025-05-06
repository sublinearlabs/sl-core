//! Util functions for the layered circuits

use p3_field::{ExtensionField, Field};
use poly::{Fields, mle::MultilinearPoly};

/// Function calculates the number of variable of the mle for a given circuit layer
pub fn compute_num_vars(layer_index: usize) -> usize {
    if layer_index == 0 {
        return 3;
    }

    let a_len = layer_index;
    let b_n_c_len = a_len + 1;

    a_len + (2 * b_n_c_len)
}

/// Function to obtain gate property
pub fn get_gate_properties(a: usize, b: usize, c: usize, layer_index: usize) -> usize {
    // Calculate the natural bit lengths
    let b_bits = (usize::BITS - b.leading_zeros()) as usize;
    let c_bits = (usize::BITS - c.leading_zeros()) as usize;

    // Determine the effective lengths after padding
    let b_len = b_bits.max(layer_index + 1);
    let c_len = c_bits.max(layer_index + 1);

    // Shift and combine using bitwise operations
    let a_shifted = a << (b_len + c_len);
    let b_shifted = b << c_len;
    a_shifted | b_shifted | c
}

/// Obtain the Multlinear poly for the MLE
pub fn mle_vec_to_poly<F: Field, E: ExtensionField<F>>(
    vec: &[usize],
    num_vars: usize,
) -> MultilinearPoly<F, E> {
    let mut mle_poly = MultilinearPoly::zero(num_vars);

    for i in vec {
        mle_poly.evaluations[*i] = Fields::Base(F::one());
    }

    mle_poly
}
