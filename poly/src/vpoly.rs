//! This module contains the implementation of the virtual polynomial.
//! A virtual polynomials is a Vector of MLEs having a combination relationship.
use std::{
    fmt::{self, Debug, Formatter},
    rc::Rc,
};

use crate::mle::MultilinearPoly;
use crate::MultilinearExtension;
use p3_field::Field;

pub struct VPoly<F: Field> {
    /// The MLEs that make up the virtual polynomial.
    mles: Vec<MultilinearPoly<F>>,
    /// max possible degree of the polynomial (This is the max number of MLE multiplication operands)
    max_degree: usize,
    /// Number of variables in the polynomial
    num_vars: usize,
    /// Combination function for evaluating the virtual polynomial.
    combine_fn: Rc<dyn Fn(&[F]) -> F>,
}

impl<F: Field + Debug> Debug for VPoly<F> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("VPoly")
            .field("mles", &self.mles)
            .field("max_degree", &self.max_degree)
            .field("num_vars", &self.num_vars)
            .field("combine_fn", &"<function>") // Display placeholder for the function
            .finish()
    }
}

impl<F: Field> VPoly<F> {
    /// Creates a new virtual polynomial from a vector of MLEs and a combination function.
    pub fn new(
        mles: Vec<MultilinearPoly<F>>,
        max_degree: usize,
        num_vars: usize,
        combine_fn: Rc<dyn Fn(&[F]) -> F>,
    ) -> Self {
        // assert all MLEs have the same number of variables
        assert!(
            mles.iter().all(|mle| mle.num_vars() == num_vars),
            "MLEs must have the same number of variables"
        );
        VPoly {
            mles,
            max_degree,
            num_vars,
            combine_fn,
        }
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

impl<F: Field> MultilinearExtension<F> for VPoly<F> {
    /// Evaluates the virtual polynomial at a given point.
    fn evaluate(&self, point: &[F]) -> F {
        let values = self
            .mles
            .iter()
            .map(|mle| mle.evaluate(point))
            .collect::<Vec<_>>();
        (self.combine_fn)(&values)
    }

    /// Partial evaluation of the virtual polynomial at a given point.
    fn partial_evaluate(&self, point: &[F]) -> Self {
        let values = self
            .mles
            .iter()
            .map(|mle| mle.partial_evaluate(point))
            .collect::<Vec<_>>();

        Self {
            mles: values,
            max_degree: self.max_degree,
            num_vars: self.num_vars - point.len(),
            combine_fn: self.combine_fn.clone(),
        }
    }

    /// Poly max degree
    fn max_degree(&self) -> usize {
        self.max_degree
    }

    // TODO: add documentation
    fn reduce(&self) -> Vec<F> {
        let mut result = vec![];
        for i in 0..(1 << self.num_vars()) {
            let row = self.mles.iter().map(|p| p[i]).collect::<Vec<F>>();
            result.push((self.combine_fn)(&row));
        }
        result
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
        (F::from_canonical_u64(2) * values[0] * values[1]) + values[2]
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
    fn test_varying_length() {
        let f_ab = MultilinearPoly::new_from_vec(
            2,
            vec![0, 0, 3, 5]
                .into_iter()
                .map(|n| F::from_canonical_u64(n))
                .collect(),
        );
        let mles = vec![f_ab, f_abc()];
        let vpoly = VPoly::new(mles, 1, 3, Rc::new(prod_combined_fn));
        let point = vec![F::from_canonical_u64(1), F::from_canonical_u64(2)];
        vpoly.partial_evaluate(&point);
    }

    #[test]
    fn test_meta_data_test() {
        let mles = vec![f_abc(), f_abc()];
        let vpoly = VPoly::new(mles, 1, 3, Rc::new(prod_combined_fn));

        assert_eq!(vpoly.num_vars(), 3);
        assert_eq!(vpoly.max_degree(), 1);
        assert_eq!(vpoly.num_mles(), 2);
    }

    #[test]
    fn test_partial_evaluation() {
        let mles = vec![f_abc(), f_abc()];
        let vpoly = VPoly::new(mles, 1, 3, Rc::new(prod_combined_fn));

        let point = vec![F::from_canonical_u64(4)];
        let expected_mles = vec![
            MultilinearPoly::new_from_vec(
                2,
                vec![0, 0, 8, 11]
                    .into_iter()
                    .map(|n| F::from_canonical_u64(n))
                    .collect(),
            ),
            MultilinearPoly::new_from_vec(
                2,
                vec![0, 0, 8, 11]
                    .into_iter()
                    .map(|n| F::from_canonical_u64(n))
                    .collect(),
            ),
        ];
        assert_eq!(vpoly.partial_evaluate(&point).mles(), expected_mles);
    }

    #[test]
    fn test_eval() {
        let mles = vec![f_abc(), f_abc()];
        let vpoly = VPoly::new(mles, 2, 3, Rc::new(prod_combined_fn)); // combination => (a * b)
        let points = vec![
            F::from_canonical_u64(1),
            F::from_canonical_u64(2),
            F::from_canonical_u64(3),
        ];
        let expected_mles = F::from_canonical_u64(22 * 22);
        assert_eq!(vpoly.evaluate(&points), expected_mles);
    }

    #[test]
    fn test_eval_1() {
        let mles = vec![f_abc(), f_abc(), f_abc()];
        let vpoly = VPoly::new(mles, 2, 3, Rc::new(combined_fn_1)); // combination => 2(a * b) + c
        let points = vec![
            F::from_canonical_u64(1),
            F::from_canonical_u64(2),
            F::from_canonical_u64(3),
        ];
        let expected_mles = F::from_canonical_u64(990);
        assert_eq!(vpoly.evaluate(&points), expected_mles);
    }
}
