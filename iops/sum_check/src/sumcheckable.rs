use p3_field::{ExtensionField, Field, PrimeField32};
use poly::mle::MultilinearPoly;
use poly::vpoly::VPoly;
use poly::{Fields, MultilinearExtension};
use transcript::Transcript;

/// Trait for structures that you can run the `Sumcheck` protocol over
pub trait Sumcheckable<F: Field, E: ExtensionField<F>> {
    /// Number of sumcheck rounds
    fn no_of_rounds(&self) -> usize;

    /// Max variable degree (determines the size of the round poly)
    fn max_var_degree(&self) -> usize;

    /// Returns the round message based on current state
    fn round_message(&self) -> Vec<Fields<F, E>>;

    /// Receives challenge and updates state
    fn receive_challenge(&mut self, challenge: &Fields<F, E>);

    /// Eval structure at some given point. Needed for the `oracle check`
    fn eval(&self, point: &[Fields<F, E>]) -> Fields<F, E>;

    /// Commit state to some transcript
    fn commit(&self, transcript: &mut Transcript<F, E>);
}

macro_rules! impl_sumcheckable_for_mle {
    ($type:ty) => {
        impl<F, E> Sumcheckable<F, E> for $type
        where
            F: Field + PrimeField32,
            E: ExtensionField<F>,
            $type: MultilinearExtension<F, E>,
        {
            fn no_of_rounds(&self) -> usize {
                self.num_vars()
            }

            fn max_var_degree(&self) -> usize {
                self.max_degree()
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
    };
}

impl_sumcheckable_for_mle!(MultilinearPoly<F, E>);
impl_sumcheckable_for_mle!(VPoly<F, E>);
