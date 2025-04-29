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
    /// Returns the sum of evaluations over the boolean hypercube
    fn sum_over_hypercube(&self) -> F;
    /// Returns the number of variables of the polynomial
    fn num_vars(&self) -> usize;
    /// Converts the polynomial to bytes
    fn to_bytes(&self) -> &[u8];
}
