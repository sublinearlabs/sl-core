//! Interface (trait) declaration of the sumcheck interface

use p3_field::{ExtensionField, Field};
use poly::Fields;

pub trait SumCheckInterface<F: Field, E: ExtensionField<F>> {
    type Polynomial;
    type Transcript;
    type Proof;
    type PartialProof;

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

    // Generates sumcheck proof for a polynomial without commiting to the initial polynomial
    // For use in GKR
    fn prove_partial(
        polynomial: &Self::Polynomial,
        transcript: &mut Self::Transcript,
    ) -> Result<Self::PartialProof, anyhow::Error>;

    // Partially verifies a sumcheck proof without knowing the initial polynomial
    // For use in GKR
    fn verify_partial(
        proof: &Self::Proof,
        transcript: &mut Self::Transcript,
    ) -> (E, Vec<Fields<F, E>>);
}
