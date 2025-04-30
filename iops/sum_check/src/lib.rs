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
            // TODO: uncomment
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
        // Appends the claimed sum to the transcript
        transcript.observe_base_element(&[proof.claimed_sum]);

        // Appends the polynomial to the transcript
        transcript.observe(polynomial.to_bytes());

        let mut claimed_sum = proof.claimed_sum;
        let mut challenges = Vec::with_capacity(polynomial.num_vars());

        // Perform round by round verification
        for round_poly in &proof.round_polynomials {
            assert_eq!(claimed_sum, round_poly[0] + round_poly[1]);
            transcript.observe_base_element(&round_poly);
            let challenge = transcript.sample_challenge();
            claimed_sum = barycentric_evaluation(&round_poly, &challenge);
            challenges.push(challenge);
        }

        // Oracle check
        assert_eq!(claimed_sum, polynomial.evaluate(&challenges));

        Ok(true)
    }
}

// Evaluate a univariate polynomial in evaluation form
pub fn barycentric_evaluation<F: Field>(evaluations: &[F], evaluation_point: &F) -> F {
    let m_x = (0..evaluations.len()).fold(F::one(), |mut acc, val| {
        acc *= *evaluation_point - F::from_canonical_usize(val);
        acc
    });

    let mut res = F::zero();

    for i in 0..evaluations.len() {
        let numerator = evaluations[i];

        let di = (0..evaluations.len())
            .into_iter()
            .filter(|val| *val != i)
            .fold(F::one(), |mut acc, val| {
                acc *= F::from_canonical_usize(i) - F::from_canonical_usize(val);
                acc
            });

        let denominator = di * (*evaluation_point - F::from_canonical_usize(i));

        res += numerator * denominator.inverse()
    }

    m_x * res
}

#[cfg(test)]
mod tests {
    use p3_mersenne_31::Mersenne31;

    use crate::barycentric_evaluation;

    #[test]
    fn test_barycentric_evaluation() {
        // Polynomial in question: 3x + 2
        let poly: Vec<Mersenne31> = [2, 3].into_iter().map(|val| Mersenne31::new(val)).collect();
        let res = barycentric_evaluation(&poly, &Mersenne31::new(5));
        assert_eq!(res, Mersenne31::new(7));

        // Polynomial in question: 5x^2 + 3x + 2
        let poly: Vec<Mersenne31> = [2, 10, 28, 56, 94]
            .into_iter()
            .map(|val| Mersenne31::new(val))
            .collect();
        let res = barycentric_evaluation(&poly, &Mersenne31::new(5));
        assert_eq!(res, Mersenne31::new(142));
    }
}
