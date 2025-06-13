//! Module holds  primitives for the sumcheck protocol

use p3_field::{ExtensionField, Field};
use poly::Fields;

pub struct SumCheckProof<F: Field, E: ExtensionField<F>> {
    pub claimed_sum: Fields<F, E>,
    pub round_polynomials: Vec<Vec<Fields<F, E>>>,
    pub challenges: Vec<Fields<F, E>>,
}

impl<F: Field, E: ExtensionField<F>> SumCheckProof<F, E> {
    pub fn new(
        claimed_sum: Fields<F, E>,
        round_polynomials: Vec<Vec<Fields<F, E>>>,
        challenges: Vec<Fields<F, E>>,
    ) -> Self {
        Self {
            claimed_sum,
            round_polynomials,
            challenges,
        }
    }
}
