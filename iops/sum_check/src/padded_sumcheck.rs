use crate::sumcheckable::Sumcheckable;
use p3_field::{ExtensionField, Field};

struct PaddedSumcheck<S> {
    inner: S,
    pad_count: usize,
}

impl<F: Field, E: ExtensionField<F>, S: Sumcheckable<F, E>> Sumcheckable<F, E>
    for PaddedSumcheck<S>
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
        todo!()
    }

    fn receive_challenge(&mut self, challenge: &poly::Fields<F, E>) {
        todo!()
    }

    fn commit(&self, transcript: &mut transcript::Transcript<F, E>) {
        todo!()
    }
}
