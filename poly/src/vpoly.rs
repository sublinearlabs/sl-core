//! This module contains the implementation of the virtual polynomial.
//! A virtual polynomials is a Vector of MLEs having a combination relationship.
use p3_field::Field;
use crate::mle::MLE;

pub struct VPoly<F: Field> {
    /// The MLEs that make up the virtual polynomial.
    mles: Vec<MLE<F>>,
    /// The function used to combine the values of the MLEs.
    combine_fn: Box<dyn Fn(&[F]) -> F>,
    /// max possible degree of the polynomial
    max_degree: usize,
}


impl<F: Field> VPoly<F> {
    /// Creates a new virtual polynomial from a vector of MLEs and a combination function.
    pub fn new(mles: Vec<MLE<F>>, combine_fn: Box<dyn Fn(&[F]) -> F>, max_degree: usize) -> Self {
        VPoly { mles, combine_fn, max_degree }
    }
}

impl<F: Field> VPoly<F> {
    /// Evaluates the virtual polynomial at a given point.
    pub fn evaluate(&self, point: &[F]) -> F {
        let values = self.mles.iter().map(|mle| mle.evaluate(point)).collect::<Vec<_>>();
        (self.combine_fn)(&values)
    }
    
    /// 
}
