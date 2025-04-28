//! This module contains the implementation of the virtual polynomial.
//! A virtual polynomials is a Vector of MLEs having a combination relationship.
use p3_field::Field;
use crate::mle::MLE;

pub struct VPoly<F: Field> {
    /// The MLEs that make up the virtual polynomial.
    mles: Vec<MLE<F>>,
    /// max possible degree of the polynomial (This is the max number of MLE multiplication operands)
    max_degree: usize,
    /// Number of variables in the polynomial
    num_vars: usize,
}


impl<F: Field> VPoly<F> {
    /// Creates a new virtual polynomial from a vector of MLEs and a combination function.
    pub fn new(mles: Vec<MLE<F>>, max_degree: usize, num_vars: usize) -> Self {
        VPoly { mles, max_degree, num_vars }
    }
    
    /// Poly max degree
    pub fn max_degree(&self) -> usize {
        self.max_degree
    }
    
    /// Number of variables in the polynomial
    pub fn num_vars(&self) -> usize {
        self.num_vars
    }
    
    /// Number of MLEs in the polynomial
    pub fn num_mles(&self) -> usize {
        self.mles.len()
    }
}

impl<F: Field> VPoly<F> {
    /// Evaluates the virtual polynomial at a given point.
    pub fn evaluate(&self, point: &[F], combine_fn: Box<dyn Fn(&[F]) -> F>) -> F {
        let values = self.mles.iter().map(|mle| mle.evaluate(point)).collect::<Vec<_>>();
        (combine_fn)(&values)
    }
    
    /// Partial evaluation of the virtual polynomial at a given point.
    pub fn partial_evalute(&self, point: &[F]) -> Self {
        let values = self.mles.iter().map(|mle| mle.partial_evalute(point)).collect::<Vec<_>>();
        
        Self {
            mles: values,
            max_degree: self.max_degree,
            num_vars: self.num_vars - point.len(),
        }
    }
}
