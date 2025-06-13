//! This module contains the implementation of the sum check protocol.
pub mod interface;
pub mod padded_sumcheck;
pub mod primitives;
pub mod sumcheckable;

use crate::sumcheckable::Sumcheckable;
use interface::SumCheckInterface;
use p3_field::{ExtensionField, Field, PrimeField32};
use poly::{utils::barycentric_evaluation, Fields};
use primitives::SumCheckProof;
use std::marker::PhantomData;
use transcript::Transcript;

pub struct SumCheck<F: Field, E: ExtensionField<F>, T: Sumcheckable<F, E> + Clone> {
    _marker: PhantomData<(F, E, T)>,
}

impl<F: Field + PrimeField32, E: ExtensionField<F>, T: Sumcheckable<F, E> + Clone>
    SumCheckInterface<F, E> for SumCheck<F, E, T>
{
    type Polynomial = T;
    type Transcript = Transcript<F, E>;
    type Proof = SumCheckProof<F, E>;

    fn prove(
        claimed_sum: Fields<F, E>,
        mut polynomial: Self::Polynomial,
        transcript: &mut Self::Transcript,
    ) -> Result<Self::Proof, anyhow::Error> {
        // Append polynomial to transcript
        polynomial.commit(transcript);

        // Append claimed sum to transcript
        transcript.observe_ext_element(&[claimed_sum.to_extension_field()]);

        SumCheck::<F, E, T>::prove_partial(claimed_sum, &mut polynomial, transcript)
    }

    fn verify(
        polynomial: &Self::Polynomial,
        proof: &Self::Proof,
        transcript: &mut Self::Transcript,
    ) -> Result<bool, anyhow::Error> {
        // Appends the polynomial to the transcript
        polynomial.commit(transcript);

        // Appends the claimed sum to the transcript
        transcript.observe_ext_element(&[proof.claimed_sum.to_extension_field()]);

        // Perform round by round verification
        let (claimed_sum, challenges) = SumCheck::<F, E, T>::verify_partial(proof, transcript);

        // Oracle check
        assert_eq!(
            claimed_sum,
            polynomial.eval(&challenges).to_extension_field()
        );

        Ok(true)
    }

    fn prove_partial(
        claimed_sum: Fields<F, E>,
        polynomial: &mut Self::Polynomial,
        transcript: &mut Self::Transcript,
    ) -> Result<Self::Proof, anyhow::Error> {
        // Init round polynomials struct
        let mut round_polynomials = Vec::with_capacity(polynomial.no_of_rounds());

        let mut challenges = vec![];

        for _ in 0..polynomial.no_of_rounds() {
            let round_message = polynomial.round_message();
            transcript.observe_ext_element(
                &round_message
                    .iter()
                    .map(|val| val.to_extension_field())
                    .collect::<Vec<E>>(),
            );
            let challenge = Fields::Extension(transcript.sample_challenge());
            polynomial.receive_challenge(&challenge);
            round_polynomials.push(round_message);
            challenges.push(challenge);
        }

        Ok(SumCheckProof::new(
            claimed_sum,
            round_polynomials,
            challenges,
        ))
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

        let proof =
            SumCheck::prove(claimed_sum, polynomial.clone(), &mut prover_transcript).unwrap();

        let mut verify_transcript = Transcript::init();
        let verify = SumCheck::verify(&polynomial, &proof, &mut verify_transcript);

        assert!(verify.unwrap());
    }
}
