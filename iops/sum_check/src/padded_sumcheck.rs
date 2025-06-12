use std::marker::PhantomData;

use crate::sumcheckable::Sumcheckable;
use p3_field::{ExtensionField, Field, PrimeField32};

struct PaddedSumcheck<F, E, S> {
    inner: S,
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
            curr_round: 0,
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

    fn eval(&self, point: &[poly::Fields<F, E>]) -> poly::Fields<F, E> {
        assert!(point.len() == self.no_of_rounds());
        self.inner.eval(&point[..self.inner.no_of_rounds()])
            * point[self.inner.no_of_rounds()..].iter().cloned().product()
    }

    fn round_message(&self) -> Vec<poly::Fields<F, E>> {
        // how do we track??
        // also how do we know when it is enough
        // tbh we need to track more things
        todo!()
    }

    fn receive_challenge(&mut self, challenge: &poly::Fields<F, E>) {
        todo!()
    }

    fn commit(&self, transcript: &mut transcript::Transcript<F, E>) {
        // commit the inner structure
        self.inner.commit(transcript);
        // commit the pad count
        transcript.observe_ext_element(&[E::from_canonical_usize(self.pad_count)])
    }
}
