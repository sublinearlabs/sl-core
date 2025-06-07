//! This module contains the implementation of the sum check protocol.
pub mod interface;
pub mod primitives;

use interface::SumCheckInterface;
use p3_field::{ExtensionField, Field, PrimeField32};
use poly::{utils::barycentric_evaluation, Fields, MultilinearExtension};
use primitives::SumCheckProof;
use std::marker::PhantomData;
use transcript::Transcript;

pub struct SumCheck<F: Field, E: ExtensionField<F>, MLE: MultilinearExtension<F, E> + Clone> {
    _marker: PhantomData<(F, E, MLE)>,
}

impl<F: Field + PrimeField32, E: ExtensionField<F>, MLE: MultilinearExtension<F, E> + Clone>
    SumCheck<F, E, MLE>
{
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<F: Field + PrimeField32, E: ExtensionField<F>, MLE: MultilinearExtension<F, E> + Clone> Default
    for SumCheck<F, E, MLE>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<F: Field + PrimeField32, E: ExtensionField<F>, MLE: MultilinearExtension<F, E> + Clone>
    SumCheckInterface<F, E> for SumCheck<F, E, MLE>
{
    type Polynomial = MLE;
    type Transcript = Transcript<F, E>;
    type Proof = SumCheckProof<F, E>;
    type PartialProof = (Vec<Vec<Fields<F, E>>>, Vec<E>);

    fn prove(
        claimed_sum: Fields<F, E>,
        polynomial: &Self::Polynomial,
        transcript: &mut Self::Transcript,
    ) -> Result<Self::Proof, anyhow::Error> {
        // Append polynomial to transcript
        polynomial.commit_to_transcript(transcript);

        // Append claimed sum to transcript
        transcript.observe_ext_element(&[claimed_sum.to_extension_field()]);

        // Generate round polynomials
        let (round_polynomials, _) =
            SumCheck::<F, E, MLE>::prove_partial(polynomial, transcript).unwrap();

        Ok(Self::Proof::new(claimed_sum, round_polynomials))
    }

    fn verify(
        polynomial: &Self::Polynomial,
        proof: &Self::Proof,
        transcript: &mut Self::Transcript,
    ) -> Result<bool, anyhow::Error> {
        // Appends the polynomial to the transcript
        polynomial.commit_to_transcript(transcript);

        // Appends the claimed sum to the transcript
        transcript.observe_ext_element(&[proof.claimed_sum.to_extension_field()]);

        // Perform round by round verification
        let (claimed_sum, challenges) = SumCheck::<F, E, MLE>::verify_partial(proof, transcript);

        // Oracle check
        assert_eq!(
            claimed_sum,
            polynomial.evaluate(&challenges).to_extension_field()
        );

        Ok(true)
    }

    fn prove_partial(
        polynomial: &Self::Polynomial,
        transcript: &mut Self::Transcript,
    ) -> Result<(Vec<Vec<Fields<F, E>>>, Vec<E>), anyhow::Error> {
        // Init round polynomials struct
        let mut round_polynomials = Vec::with_capacity(polynomial.num_vars());

        let mut poly = polynomial.clone();

        let mut challenges = vec![];

        for _ in 0..poly.num_vars() {
            let mut round_poly = Vec::with_capacity(poly.max_degree());
            for point in 0..=poly.max_degree() {
                let value = poly
                    .partial_evaluate(&[Fields::Extension(E::from_canonical_usize(point))])
                    .sum_over_hypercube();
                round_poly.push(value);
            }
            transcript.observe_ext_element(
                &round_poly
                    .iter()
                    .map(|val| val.to_extension_field())
                    .collect::<Vec<E>>(),
            );
            let challenge = transcript.sample_challenge();
            poly = poly.partial_evaluate(&[Fields::Extension(challenge)]);
            round_polynomials.push(round_poly);
            challenges.push(challenge);
        }

        Ok((round_polynomials, challenges))
    }

    fn verify_partial(
        proof: &Self::Proof,
        transcript: &mut Self::Transcript,
    ) -> (E, Vec<Fields<F, E>>) {
        let mut claimed_sum = proof.claimed_sum.to_extension_field();

        let mut challenges = vec![];

        // Perform round by round verification
        for round_poly in &proof.round_polynomials {
            assert_eq!(
                claimed_sum,
                round_poly[0].to_extension_field() + round_poly[1].to_extension_field()
            );
            transcript.observe_ext_element(
                &round_poly
                    .iter()
                    .map(|val| val.to_extension_field())
                    .collect::<Vec<E>>(),
            );
            let challenge = Fields::Extension(transcript.sample_challenge());
            claimed_sum = barycentric_evaluation(round_poly, &challenge).to_extension_field();
            challenges.push(challenge);
        }

        (claimed_sum, challenges)
    }
}

#[cfg(test)]
mod tests {
    use crate::{SumCheck, SumCheckInterface};
    use p3_field::extension::BinomialExtensionField;
    use p3_mersenne_31::Mersenne31;
    use poly::{mle::MultilinearPoly, Fields, MultilinearExtension};
    use transcript::Transcript;

    type F = Mersenne31;
    type E = BinomialExtensionField<Mersenne31, 3>;

    fn f_abc() -> MultilinearPoly<F, E> {
        MultilinearPoly::new_from_vec(
            3,
            vec![0, 0, 0, 3, 0, 0, 2, 5]
                .into_iter()
                .map(|val| Fields::Base(F::new(val)))
                .collect(),
        )
    }

    #[test]
    fn test_sumcheck() {
        let polynomial = f_abc();

        let claimed_sum = polynomial.sum_over_hypercube();
        let mut prover_transcript = Transcript::init();

        let proof = SumCheck::prove(claimed_sum, &polynomial, &mut prover_transcript).unwrap();

        let mut verify_transcript = Transcript::init();
        let verify = SumCheck::verify(&polynomial, &proof, &mut verify_transcript);

        assert!(verify.unwrap());
    }
}
