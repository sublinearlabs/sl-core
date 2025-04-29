pub mod mle;
pub mod vpoly;

/// Multilinear Extension Trait
pub trait MultilinearExtension<F> {
    /// Fix all variables
    fn evaluate(&self, point: &[F]) -> F;
    /// Partially fix variables starting from the first
    fn partial_evaluate(&self, point: &[F]) -> Self;
    /// Returns the max variable degree
    fn max_degree(&self) -> usize;
    /// Returns the boolean hypercube evaluations of the structure
    fn reduce(&self) -> Vec<F>;
}
