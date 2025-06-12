use std::marker::PhantomData;

use crate::Fields;
use crate::sumcheckable::Sumcheckable;
use p3_field::{ExtensionField, Field, PrimeField32};

struct PaddedSumcheck<F, E, S> {
    inner: S,
    eval: Option<E>,
    n: usize,
    pad_count: usize,
    curr_round: usize,
    _marker: PhantomData<(F, E)>,
}

impl<F: Field, E: ExtensionField<F>, S: Sumcheckable<F, E>> PaddedSumcheck<F, E, S> {
    fn new(inner: S, pad_count: usize) -> Self {
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
            (0..self.max_var_degree())
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
    }

    fn commit(&self, transcript: &mut transcript::Transcript<F, E>) {
        // commit the inner structure
        self.inner.commit(transcript);
        // commit the pad count
        transcript.observe_ext_element(&[E::from_canonical_usize(self.pad_count)])
    }
}
