//! This module contains the implementation of the sum check protocol.
use p3_challenger::FieldChallenger;
use p3_field::{ExtensionField, Field};
use std::marker::PhantomData;
use transcript::Transcript;

pub trait SumCheckInterface<F: Field> {
    type Polynomial;
    type Transcript;
    type Proof;

    /// Generate proof for a polynomial sum over the bolean hypercube
    fn prove(
        polynomial: Self::Polynomial,
        transcript: &mut Self::Transcript,
    ) -> Result<Self::Proof, anyhow::Error>;

    /// Verify proof for a polynomial sum over the bolean hypercube
    fn verify(
        sum: F,
        proof: Self::Proof,
        transcript: &mut Self::Transcript,
    ) -> Result<bool, anyhow::Error>;
}

pub struct SumCheck<F: Field, E: ExtensionField<F>, FC: FieldChallenger<F>> {
    _marker: PhantomData<(F, E, FC)>,
}

impl<F: Field, E: ExtensionField<F>, FC: FieldChallenger<F>> SumCheck<F, E, FC> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<F: Field, E: ExtensionField<F>, FC: FieldChallenger<F>> SumCheckInterface<F>
    for SumCheck<F, E, FC>
{
    type Polynomial = ();
    type Transcript = Transcript<F, E, FC>;
    type Proof = ();

    fn prove(
        polynomial: Self::Polynomial,
        transcript: &mut Self::Transcript,
    ) -> Result<Self::Proof, anyhow::Error> {
        todo!()
    }

    fn verify(
        sum: F,
        proof: Self::Proof,
        transcript: &mut Self::Transcript,
    ) -> Result<bool, anyhow::Error> {
        todo!()
    }
}
