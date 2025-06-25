pub mod mle;
pub mod utils;
pub mod vpoly;

/// Multilinear Extension Trait
pub trait MultilinearExtension<F: Field, E: ExtensionField<F>> {
    /// Fix all variables
    fn evaluate(&self, point: &[Fields<F, E>]) -> Fields<F, E>;
    /// Partially fix variables starting from the first
    fn partial_evaluate(&self, point: &[Fields<F, E>]) -> Self;
    /// Returns the max variable degree
    fn max_degree(&self) -> usize;
    /// Returns the sum of evaluations over the boolean hypercube
    fn sum_over_hypercube(&self) -> Fields<F, E>;
    /// Returns the number of variables of the polynomial
    fn num_vars(&self) -> usize;
    /// Commit structure to transcript
    fn commit_to_transcript(&self, transcript: &mut transcript::Transcript<F, E>)
    where
        F: PrimeField32;
}
