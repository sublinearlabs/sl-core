use std::marker::PhantomData;

use crate::sumcheckable::Sumcheckable;
use crate::Fields;
use p3_field::{ExtensionField, Field, PrimeField32};

#[derive(Clone)]
/// Pads a polynomial with extra variables in the following form
/// f(a, b, c) with pad_count = 2 will give
/// f'(a, b, c, d, e) = d * e * f(a, b, c)
/// see: https://hackmd.io/@iammadab/BJJiocNfel
pub struct PaddedSumcheck<F, E, S> {
    inner: S,
    eval: Option<E>,
    n: usize,
    pad_count: usize,
    curr_round: usize,
    _marker: PhantomData<(F, E)>,
}

impl<F: Field, E: ExtensionField<F>, S: Sumcheckable<F, E>> PaddedSumcheck<F, E, S> {
    pub fn new(inner: S, pad_count: usize) -> Self {
        Self {
            n: inner.no_of_rounds(),
            pad_count,
            inner,
            curr_round: 1,
            eval: None,
            _marker: PhantomData,
        }
    }
}

impl<F: Field + PrimeField32, E: ExtensionField<F>, S: Sumcheckable<F, E>> Sumcheckable<F, E>
    for PaddedSumcheck<F, E, S>
{
    fn no_of_rounds(&self) -> usize {
        self.inner.no_of_rounds() + self.pad_count
    }

    fn max_var_degree(&self) -> usize {
        self.inner.max_var_degree()
    }

    fn eval(&self, point: &[Fields<F, E>]) -> Fields<F, E> {
        assert!(point.len() == self.no_of_rounds());
        self.inner.eval(&point[..self.inner.no_of_rounds()])
            * point[self.inner.no_of_rounds()..].iter().cloned().product()
    }

    fn round_message(&self) -> Vec<Fields<F, E>> {
        if self.curr_round <= self.n {
            self.inner.round_message()
        } else {
            (0..=self.max_var_degree() + 1)
                .map(|i| Fields::Extension(E::from_canonical_usize(i) * self.eval.unwrap()))
                .collect()
        }
    }

    fn receive_challenge(&mut self, challenge: &Fields<F, E>) {
        if self.curr_round < self.n {
            self.inner.receive_challenge(challenge);
        } else if self.curr_round == self.n {
            let claimed_sum = self.inner.eval(&[*challenge]);
            self.eval = Some(claimed_sum.to_extension_field());
        } else {
            self.eval = Some(self.eval.unwrap() * challenge.to_extension_field());
        }

        self.curr_round += 1;
    }

    fn commit(&self, transcript: &mut transcript::Transcript<F, E>) {
        // commit the inner structure
        self.inner.commit(transcript);
        // commit the pad count
        transcript.observe_ext_element(&[E::from_canonical_usize(self.pad_count)])
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use p3_field::{extension::BinomialExtensionField, AbstractField};
    use p3_mersenne_31::Mersenne31 as F;
    type E = BinomialExtensionField<F, 3>;

    use poly::vpoly::VPoly;
    use poly::{mle::MultilinearPoly, Fields, MultilinearExtension};
    use transcript::Transcript;

    use crate::sumcheckable::Sumcheckable;
    use crate::SumCheck;
    use crate::SumCheckInterface;

    use super::PaddedSumcheck;

    fn to_fields(vals: Vec<u64>) -> Vec<Fields<F, E>> {
        vals.into_iter()
            .map(|v| Fields::<F, E>::Base(F::from_canonical_u64(v)))
            .collect()
    }

    fn f_abc() -> MultilinearPoly<F, E> {
        // f(a, b, c) = 2ab + 3bc
        MultilinearPoly::new_from_vec(3, to_fields(vec![0, 0, 0, 3, 0, 0, 2, 5]))
    }

    fn prod_combined_fn(values: &[Fields<F, E>]) -> Fields<F, E> {
        Fields::Extension(values[0].to_extension_field() * values[1].to_extension_field())
    }

    #[test]
    fn test_padded_sumcheck_eval() {
        let poly = f_abc();
        let padded_poly = PaddedSumcheck::new(poly, 2);
        assert_eq!(padded_poly.no_of_rounds(), 5);
        // TODO: why does the evaluation return an extension field when all
        //  the inputs are base field elements
        assert_eq!(
            padded_poly.eval(&to_fields(vec![4, 3, 2, 5, 0])),
            Fields::Extension(E::zero())
        );

        assert_eq!(
            // a = 3, b = 4, c = 2
            // (2ab + 3bc) * d * e
            // (2 * 3 * 4 + 3 * 4 * 2) * 3 * 4
            // (24 + 24) * 12 = 48 * 12 = 576
            padded_poly.eval(&to_fields(vec![3, 4, 2, 3, 4])),
            Fields::Extension(E::from_canonical_u64(576))
        )
    }

    #[test]
    fn test_multilinear_poly_padded_sumcheck() {
        let poly = f_abc();

        let claimed_sum = poly.sum_over_hypercube();
        let padded_poly = PaddedSumcheck::new(poly, 4);
        assert_eq!(padded_poly.no_of_rounds(), 7);

        let mut prover_transcript = Transcript::init();
        let proof =
            SumCheck::prove(claimed_sum, padded_poly.clone(), &mut prover_transcript).unwrap();

        let mut verify_transcript = Transcript::init();
        let verification_result = SumCheck::verify(&padded_poly, &proof, &mut verify_transcript);

        assert!(verification_result.unwrap());
    }

    #[test]
    fn test_v_poly_padded_sumcheck() {
        // g(a, b, c) = f(a, b, c) * f(a, b, c)
        let poly = VPoly::new(vec![f_abc(), f_abc()], 2, 3, Rc::new(prod_combined_fn));
        // f(a, b, c) = 2ab + 3bc
        // f(1, 2, 3) = 2(1)(2) + 3(2)(3) = 4 + 18 = 22
        assert_eq!(
            poly.eval(&to_fields(vec![1, 2, 3])),
            Fields::Extension(E::from_canonical_u64(484))
        );

        let claimed_sum = poly.sum_over_hypercube();
        let padded_poly = PaddedSumcheck::new(poly, 10);
        assert_eq!(padded_poly.no_of_rounds(), 13);

        let mut prover_transcript = Transcript::init();
        let proof =
            SumCheck::prove(claimed_sum, padded_poly.clone(), &mut prover_transcript).unwrap();

        // ensure that round poly takes into account the variable degree
        assert_eq!(proof.round_polynomials[0].len(), 3);

        let mut verify_transcript = Transcript::init();
        let verification_result = SumCheck::verify(&padded_poly, &proof, &mut verify_transcript);

        assert!(verification_result.unwrap());
    }
}
