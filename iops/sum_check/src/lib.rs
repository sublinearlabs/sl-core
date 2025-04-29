//! This module contains the implementation of the sum check protocol.
use anyhow::Ok;
use p3_challenger::FieldChallenger;
use p3_field::{ExtensionField, Field};
use poly::MultilinearExtension;
use std::marker::PhantomData;
use transcript::Transcript;

pub struct SumCheckProof<F: Field> {
    pub claimed_sum: F,
    pub round_polynomials: Vec<Vec<F>>,
}

impl<F: Field> SumCheckProof<F> {
    pub fn new(claimed_sum: F, round_polynomials: Vec<Vec<F>>) -> Self {
        Self {
            claimed_sum,
            round_polynomials,
        }
    }
}

pub trait SumCheckInterface<F: Field> {
    type Polynomial;
    type Transcript;
    type Proof;

    /// Generate proof for a polynomial sum over the bolean hypercube
    fn prove(
        polynomial: &Self::Polynomial,
        transcript: &mut Self::Transcript,
    ) -> Result<Self::Proof, anyhow::Error>;

    /// Verify proof for a polynomial sum over the bolean hypercube
    fn verify(
        polynomial: &Self::Polynomial,
        proof: &Self::Proof,
        transcript: &mut Self::Transcript,
    ) -> Result<bool, anyhow::Error>;
}

pub struct SumCheck<
    F: Field,
    E: ExtensionField<F>,
    FC: FieldChallenger<F>,
    MLE: MultilinearExtension<F>,
> {
    _marker: PhantomData<(F, E, FC, MLE)>,
}

impl<F: Field, E: ExtensionField<F>, FC: FieldChallenger<F>, MLE: MultilinearExtension<F>>
    SumCheck<F, E, FC, MLE>
{
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<F: Field, E: ExtensionField<F>, FC: FieldChallenger<F>, MLE: MultilinearExtension<F>>
    SumCheckInterface<F> for SumCheck<F, E, FC, MLE>
{
    type Polynomial = MLE;
    type Transcript = Transcript<F, E, FC>;
    type Proof = SumCheckProof<F>;

    fn prove(
        polynomial: &Self::Polynomial,
        transcript: &mut Self::Transcript,
    ) -> Result<Self::Proof, anyhow::Error> {
        // calculate the sum over the boolean hypercube
        let claimed_sum = polynomial.sum_over_hypercube();

        // Init round polynomials struct
        let mut round_polynomials = Vec::with_capacity(polynomial.num_vars());

        // Append claimed sum to transcript
        transcript.observe_base_element(&[claimed_sum]);

        // Append polynomial to transcript
        transcript.observe(polynomial.to_bytes());

        let mut poly = polynomial;

        for _ in 0..poly.num_vars() {
            let mut round_poly = Vec::with_capacity(poly.max_degree());
            for point in 0..poly.max_degree() {
                let value = poly
                    .partial_evaluate(&[F::from_canonical_usize(point)])
                    .sum_over_hypercube();
                round_poly.push(value);
            }
            let challenge = transcript.sample_challenge();
            poly = &poly.partial_evaluate(&[challenge]);
            round_polynomials.push(round_poly);
        }

        Ok(Self::Proof::new(claimed_sum, round_polynomials))
    }

    fn verify(
        polynomial: &Self::Polynomial,
        proof: &Self::Proof,
        transcript: &mut Self::Transcript,
    ) -> Result<bool, anyhow::Error> {
        todo!()
    }
}
