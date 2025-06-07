use p3_field::{ExtensionField, Field};
use poly::Fields;
use transcript::Transcript;

// TODO: add documentation
trait Sumcheckable<F: Field, E: ExtensionField<F>> {
    // TODO: can we be more strict with the number of elements in the round poly at this layer??
    //  enforce correctness at the type level
    fn round_message(&self) -> Vec<F>;

    // TODO: add documentation
    fn receive_challenge(&mut self, challenge: Fields<F, E>);

    // TODO: add documentation
    fn eval(&self, point: &[Fields<F, E>]) -> Fields<F, E>;

    // TODO: add documentation
    fn commit_to_transcript(&self, transcript: &mut Transcript<F, E>);
}
