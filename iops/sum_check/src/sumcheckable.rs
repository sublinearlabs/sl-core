use p3_field::{ExtensionField, Field, PrimeField32};
use poly::{Fields, MultilinearExtension};
use transcript::Transcript;

// TODO: add documentation
pub trait Sumcheckable<F: Field, E: ExtensionField<F>> {
    // TODO: add documentation
    fn no_of_rounds(&self) -> usize;

    // TODO: can we be more strict with the number of elements in the round poly at this layer??
    //  enforce correctness at the type level
    fn round_message(&self) -> Vec<Fields<F, E>>;

    // TODO: add documentation
    fn receive_challenge(&mut self, challenge: &Fields<F, E>);

    // TODO: add documentation
    fn eval(&self, point: &[Fields<F, E>]) -> Fields<F, E>;

    // TODO: add documentation
    fn commit(&self, transcript: &mut Transcript<F, E>);
}

impl<F, E, MLE> Sumcheckable<F, E> for MLE
where
    F: Field + PrimeField32,
    E: ExtensionField<F>,
    MLE: MultilinearExtension<F, E>,
{
    fn no_of_rounds(&self) -> usize {
        self.num_vars()
    }

    fn eval(&self, point: &[Fields<F, E>]) -> Fields<F, E> {
        self.evaluate(point)
    }

    fn commit(&self, transcript: &mut Transcript<F, E>) {
        self.commit_to_transcript(transcript);
    }

    fn receive_challenge(&mut self, challenge: &Fields<F, E>) {
        *self = self.partial_evaluate(&[*challenge]);
    }

    fn round_message(&self) -> Vec<Fields<F, E>> {
        (0..=self.max_degree())
            .map(|p| Fields::Extension(E::from_canonical_usize(p)))
            .map(|p| self.partial_evaluate(&[p]).sum_over_hypercube())
            .collect()
    }
}
