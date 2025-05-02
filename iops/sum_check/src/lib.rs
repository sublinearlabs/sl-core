//! This module contains the implementation of the sum check protocol.
use anyhow::Ok;
use p3_challenger::FieldChallenger;
use p3_field::{ExtensionField, Field};
use poly::{Fields, MultilinearExtension};
use std::marker::PhantomData;
use transcript::Transcript;

pub struct SumCheckProof<F: Field, E: ExtensionField<F>> {
    pub claimed_sum: Fields<F, E>,
    pub round_polynomials: Vec<Vec<Fields<F, E>>>,
}

impl<F: Field, E: ExtensionField<F>> SumCheckProof<F, E> {
    pub fn new(claimed_sum: Fields<F, E>, round_polynomials: Vec<Vec<Fields<F, E>>>) -> Self {
        Self {
            claimed_sum,
            round_polynomials,
        }
    }
}

pub trait SumCheckInterface<F: Field, E: ExtensionField<F>> {
    type Polynomial;
    type Transcript;
    type Proof;

    /// Generate proof for a polynomial sum over the bolean hypercube
    fn prove(
        claimed_sum: Fields<F, E>,
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
    MLE: MultilinearExtension<F, E> + Clone,
> {
    _marker: PhantomData<(F, E, FC, MLE)>,
}

impl<
    F: Field,
    E: ExtensionField<F>,
    FC: FieldChallenger<F>,
    MLE: MultilinearExtension<F, E> + Clone,
> SumCheck<F, E, FC, MLE>
{
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<
    F: Field,
    E: ExtensionField<F>,
    FC: FieldChallenger<F>,
    MLE: MultilinearExtension<F, E> + Clone,
> SumCheckInterface<F, E> for SumCheck<F, E, FC, MLE>
{
    type Polynomial = MLE;
    type Transcript = Transcript<F, E, FC>;
    type Proof = SumCheckProof<F, E>;

    fn prove(
        claimed_sum: Fields<F, E>,
        polynomial: &Self::Polynomial,
        transcript: &mut Self::Transcript,
    ) -> Result<Self::Proof, anyhow::Error> {
        // Init round polynomials struct
        let mut round_polynomials = Vec::with_capacity(polynomial.num_vars());

        // Append claimed sum to transcript
        transcript.observe_ext_element(&[claimed_sum.to_extension_field()]);

        // Append polynomial to transcript
        polynomial.commit_to_transcript(transcript);

        let mut poly = polynomial.clone();

        for i in 0..poly.num_vars() {
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
            dbg!("Prover: ", i, challenge);
            poly = poly.partial_evaluate(&[Fields::Extension(challenge)]);
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
        transcript.observe_ext_element(&[proof.claimed_sum.to_extension_field()]);

        // Appends the polynomial to the transcript
        polynomial.commit_to_transcript(transcript);

        let mut claimed_sum = proof.claimed_sum.to_extension_field();
        let mut challenges = Vec::with_capacity(polynomial.num_vars());

        let mut i = 0;
        // Perform round by round verification
        for round_poly in &proof.round_polynomials {
            assert_eq!(
                claimed_sum,
                round_poly[0].to_extension_field() + round_poly[1].to_extension_field()
            );
            transcript.observe_ext_element(
                &round_poly
                    .into_iter()
                    .map(|val| val.to_extension_field())
                    .collect::<Vec<E>>(),
            );
            let challenge = Fields::Extension(transcript.sample_challenge());
            dbg!("Verifier: ", i, challenge);
            i += 1;
            claimed_sum = barycentric_evaluation(&round_poly, &challenge).to_extension_field();
            challenges.push(challenge);
        }

        // Oracle check
        assert_eq!(
            claimed_sum,
            polynomial.evaluate(&challenges).to_extension_field()
        );

        Ok(true)
    }
}

// Evaluate a univariate polynomial in evaluation form
pub fn barycentric_evaluation<F: Field, E: ExtensionField<F>>(
    evaluations: &[Fields<F, E>],
    evaluation_point: &Fields<F, E>,
) -> Fields<F, E> {
    let m_x = (0..evaluations.len()).fold(E::one(), |mut acc, val| {
        acc *= evaluation_point.to_extension_field() - E::from_canonical_usize(val);
        acc
    });

    let mut res = E::zero();

    for i in 0..evaluations.len() {
        let numerator = evaluations[i].to_extension_field();

        let di = (0..evaluations.len())
            .into_iter()
            .filter(|val| *val != i)
            .fold(E::one(), |mut acc, val| {
                acc *= F::from_canonical_usize(i) - F::from_canonical_usize(val);
                acc
            });

        let denominator = di * (evaluation_point.to_extension_field() - E::from_canonical_usize(i));

        res += numerator * denominator.inverse()
    }

    Fields::Extension(m_x * res)
}

#[cfg(test)]
mod tests {
    use p3_challenger::{HashChallenger, SerializingChallenger32};
    use p3_field::{AbstractExtensionField, extension::BinomialExtensionField};
    use p3_keccak::Keccak256Hash;
    use p3_mersenne_31::Mersenne31;
    use poly::{Fields, MultilinearExtension, mle::MultilinearPoly};
    use transcript::Transcript;

    use crate::{SumCheck, SumCheckInterface, SumCheckProof, barycentric_evaluation};

    type F = Mersenne31;

    type E = BinomialExtensionField<Mersenne31, 3>;

    type FC = SerializingChallenger32<Mersenne31, HashChallenger<u8, Keccak256Hash, 32>>;

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
    fn test_barycentric_evaluation() {
        // Polynomial in question: 3x + 2
        let poly: Vec<Fields<Mersenne31, BinomialExtensionField<Mersenne31, 3>>> = [2, 3]
            .into_iter()
            .map(|val| Fields::Base(Mersenne31::new(val)))
            .collect();
        let res = barycentric_evaluation(&poly, &Fields::Base(Mersenne31::new(5)));
        assert_eq!(
            res,
            Fields::Extension(AbstractExtensionField::from_base(Mersenne31::new(7)))
        );

        // Polynomial in question: 5x^2 + 3x + 2
        let poly: Vec<Fields<Mersenne31, BinomialExtensionField<Mersenne31, 3>>> =
            [2, 10, 28, 56, 94]
                .into_iter()
                .map(|val| Fields::Base(Mersenne31::new(val)))
                .collect();
        let res = barycentric_evaluation(&poly, &Fields::Base(Mersenne31::new(5)));
        assert_eq!(
            res,
            Fields::Extension(AbstractExtensionField::from_base(Mersenne31::new(142)))
        );
    }

    #[test]
    fn test_sumcheck() {
        let polynomial = f_abc();

        let claimed_sum = polynomial.sum_over_hypercube();

        let challenger = FC::new(HashChallenger::new(vec![], Keccak256Hash));

        let mut prover_transcript = Transcript::init_with_challenger(challenger.clone());

        let proof: SumCheckProof<Mersenne31, BinomialExtensionField<Mersenne31, 3>> =
            SumCheck::prove(claimed_sum, &polynomial, &mut prover_transcript).unwrap();

        let mut verify_transcript = Transcript::init_with_challenger(challenger);

        let verify = SumCheck::verify(&polynomial, &proof, &mut verify_transcript);

        assert!(verify.unwrap());
    }
}
