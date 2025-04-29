//! This module contains the implementation of the sum check protocol.
use p3_field::Field;
use std::marker::PhantomData;

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

pub struct SumCheck<F: Field> {
    _marker: PhantomData<F>,
}

impl<F: Field> SumCheck<F> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<F: Field> SumCheckInterface<F> for SumCheck<F> {
    type Polynomial = ();
    type Transcript = ();
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
