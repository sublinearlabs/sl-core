use p3_field::{AbstractField, PrimeField32};
use poly::{Fields, MultilinearExtension};
use transcript::Transcript;

pub trait Sumcheckable {
    /// Assumes we are running sumcheck over a structure that outputs
    /// uniformed type elements also process same type.
    /// Item represents that type for the given structure.
    type Item: Clone;
    /// Transcript type to allow for structure commitment
    type Transcript;

    /// Number of sumcheck rounds
    fn no_of_rounds(&self) -> usize;

    /// Returns the round message based on current state
    fn round_message(&self) -> Vec<Self::Item>;

    /// Receives challenge and updates state
    fn receive_challenge(&mut self, challenge: &Self::Item);

    /// Eval structure at some given point. Needed for the `oracle check`
    fn eval(&self, point: &[Self::Item]) -> Self::Item;

    // TODO: deeply contemplate if this is the right place to put the transcript
    //  definitions (doesn't feel right, something better could be done)

    /// commit structure state to some transacript
    fn commit(&self, transcript: &mut Self::Transcript);

    /// commit array of items
    fn commit_items(items: &[Self::Item], transcript: &mut Self::Transcript);

    /// sample challenge
    fn sample_challenge(transcript: &mut Self::Transcript) -> Self::Item;
}

impl<T: MultilinearExtension> Sumcheckable for T
where
    T::F: PrimeField32,
{
    type Item = Fields<T::F, T::E>;
    type Transcript = Transcript<T::F, T::E>;

    fn no_of_rounds(&self) -> usize {
        self.num_vars()
    }

    fn round_message(&self) -> Vec<Self::Item> {
        (0..=self.max_degree())
            .map(|p| Fields::Extension(T::E::from_canonical_usize(p)))
            .map(|p| self.partial_evaluate(&[p]).sum_over_hypercube())
            .collect()
    }

    fn receive_challenge(&mut self, challenge: &Self::Item) {
        *self = self.partial_evaluate(&[*challenge]);
    }

    fn eval(&self, point: &[Self::Item]) -> Self::Item {
        self.evaluate(point)
    }

    fn commit(&self, transcript: &mut Self::Transcript) {
        self.commit_to_transcript(transcript);
    }

    fn commit_items(items: &[Self::Item], transcript: &mut Self::Transcript) {
        transcript.observe_ext_element(
            &items
                .iter()
                .map(|t| t.to_extension_field())
                .collect::<Vec<_>>(),
        )
    }

    fn sample_challenge(transcript: &mut Self::Transcript) -> Self::Item {
        Fields::Extension(transcript.sample_challenge())
    }
}
