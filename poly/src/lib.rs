pub mod mle;
pub mod vpoly;

pub trait MultilinearExtension<F> {
    fn evaluate(&self, point: &[F]) -> F;
    fn partial_evaluate(&self, point: &[F]) -> Self;
    fn max_degree(&self) -> usize;
    fn reduce(&self) -> &[F];
}
