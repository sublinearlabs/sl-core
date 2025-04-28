//! This module contains the implementation of the virtual polynomial.
//! A virtual polynomials is a Vector of MLEs having a combination relationship.
use p3_field::Field;
use crate::mle::MultilinearPoly;

#[derive(Debug, Clone)]
pub struct VPoly<F: Field> {
    /// The MLEs that make up the virtual polynomial.
    mles: Vec<MultilinearPoly<F>>,
    /// max possible degree of the polynomial (This is the max number of MLE multiplication operands)
    max_degree: usize,
    /// Number of variables in the polynomial
    num_vars: usize,
}


impl<F: Field> VPoly<F> {
    /// Creates a new virtual polynomial from a vector of MLEs and a combination function.
    pub fn new(mles: Vec<MultilinearPoly<F>>, max_degree: usize, num_vars: usize) -> Self {
        // assert all MLEs have the same number of variables
        assert!(mles.iter().all(|mle| mle.num_vars() == num_vars), "MLEs must have the same number of variables");
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
    
    /// MLEs in the polynomial
    pub fn mles(&self) -> Vec<MultilinearPoly<F>> {
        self.mles.clone()
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


#[cfg(test)]
mod tests {
    use super::*;
    use p3_field::AbstractField;
    use p3_goldilocks::Goldilocks as F;
    
    fn prod_combined_fn(values: &[F]) -> F {
        values[0] * values[1]
    }
    
    fn combined_fn_1(values: &[F]) -> F {
        (F::from_canonical_u64(2) * values[0] * values[1]) +  values[2]
    }
    
    fn f_abc() -> MultilinearPoly<F> {
        // f(a,b,c) = 2ab + 3bc;
        MultilinearPoly::new_from_vec(
            3,
            vec![0, 0, 0, 3, 0, 0, 2, 5]
                .into_iter()
                .map(|n| F::from_canonical_u64(n))
                .collect(),
        )
    }
    
    #[test]
    #[should_panic = "MLEs must have the same number of variables"]
    fn test_varying_lenght() {
        let f_ab = MultilinearPoly::new_from_vec(
            2,
            vec![0, 0, 3, 5]
                .into_iter()
                .map(|n| F::from_canonical_u64(n))
                .collect(),
        );
        let mles = vec![f_ab, f_abc()];
        let vpoly = VPoly::new(mles, 1, 3);
        let point = vec![F::from_canonical_u64(1), F::from_canonical_u64(2)];
        vpoly.partial_evalute(&point);
    }
    
    #[test]
    fn test_meta_data_test() {
        let mles = vec![f_abc(), f_abc()];
        let vpoly = VPoly::new(mles, 1, 3);
        
        
        assert_eq!(vpoly.num_vars(), 3);
        assert_eq!(vpoly.max_degree(), 1);
        assert_eq!(vpoly.num_mles(), 2);
    }
    
    #[test]
    fn test_partial_evaluation() {
        let mles = vec![f_abc(), f_abc()];
        let vpoly = VPoly::new(mles, 1, 3);
        
        let point = vec![F::from_canonical_u64(4)];
        let expected_mles = vec![MultilinearPoly::new_from_vec(
            2,
            vec![0, 0, 8, 11]
                .into_iter()
                .map(|n| F::from_canonical_u64(n))
                .collect(),
        ), MultilinearPoly::new_from_vec(
            2,
            vec![0, 0, 8, 11]
                .into_iter()
                .map(|n| F::from_canonical_u64(n))
                .collect(),
        )];
        assert_eq!(vpoly.partial_evalute(&point).mles(), expected_mles);
    }
    
    #[test]
    fn test_eval() {
        let mles = vec![f_abc(), f_abc()];
        let vpoly = VPoly::new(mles, 2, 3); // combination => (a * b)
        let points = vec![F::from_canonical_u64(1), F::from_canonical_u64(2), F::from_canonical_u64(3)];
        let expected_mles = F::from_canonical_u64(22 * 22);
        assert_eq!(vpoly.evaluate(&points, Box::new(prod_combined_fn)), expected_mles);
    }
    
    #[test]
    fn test_eval_1() {
        let mles = vec![f_abc(), f_abc(), f_abc()];
        let vpoly = VPoly::new(mles, 2, 3); // combination => 2(a * b) + c
        let points = vec![F::from_canonical_u64(1), F::from_canonical_u64(2), F::from_canonical_u64(3)];
        let expected_mles = F::from_canonical_u64(990);
        assert_eq!(vpoly.evaluate(&points, Box::new(combined_fn_1)), expected_mles);
    }
}