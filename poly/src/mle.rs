use std::ops::Index;

use p3_field::Field;

use crate::MultilinearExtension;

#[derive(Debug, Clone, PartialEq)]
pub struct MultilinearPoly<F: Field> {
    /// The evaluations of the boolean hypercube {0,1}^n_vars
    evaluations: Vec<F>,
    /// Number of variables
    n_vars: usize,
}

impl<F: Field> MultilinearPoly<F> {
    /// Instantiates a `MultilinearPoly` from a vec of field elements
    pub fn new_from_vec(n_vars: usize, evaluations: Vec<F>) -> Self {
        // assert that the number of variables matches the number of evaluations
        assert_eq!(1 << n_vars, evaluations.len());
        Self {
            evaluations,
            n_vars,
        }
    }

    /// Number of variables in the `MultilinearPoly`
    pub fn num_vars(&self) -> usize {
        self.n_vars
    }
}

impl<F: Field> MultilinearExtension<F> for MultilinearPoly<F> {
    /// Partially fixes variables in the `MultilinearPoly`
    /// Returns a new `MultilinearPoly` after fixed variables have
    /// been removed
    fn partial_evaluate(&self, points: &[F]) -> Self {
        // ensure we don't have more points than variables
        assert!(points.len() <= self.n_vars);

        let mut new_evaluations = self.evaluations.clone();

        // for each partial point, fold the evaluations in half
        let mut mid_point = new_evaluations.len() / 2;
        for point in points {
            for i in 0..mid_point {
                let left = new_evaluations[i];
                let right = new_evaluations[i + mid_point];
                new_evaluations[i] = match point {
                    // if the evaluation point is in the boolean hypercube
                    // return result from table directly
                    a if a.is_zero() => left,
                    a if a.is_one() => right,

                    // linear interpolation
                    // (1-r) * left + r * right
                    // left - r.left + r.right
                    // left - r (left - right)
                    _ => left - *point * (left - right),
                }
            }
            mid_point /= 2;
        }

        // truncate and return new polynomial
        let n_vars = self.n_vars - points.len();
        Self {
            evaluations: new_evaluations[..(1 << n_vars)].to_vec(),
            n_vars,
        }
    }

    /// Fixes all variables in the `MultilinearPoly` return a single
    /// field element
    fn evaluate(&self, points: &[F]) -> F {
        // ensure number of points exactly matches number of variables
        assert_eq!(self.n_vars, points.len());
        self.partial_evaluate(points).evaluations[0]
    }

    /// Polynomial max variable degree
    fn max_degree(&self) -> usize {
        1
    }

    // TODO: add documentation
    fn reduce(&self) -> Vec<F> {
        // TODO: get rid of this clone
        self.evaluations.clone()
    }
}

impl<F: Field> Index<usize> for MultilinearPoly<F> {
    type Output = F;

    fn index(&self, index: usize) -> &Self::Output {
        &self.evaluations[index]
    }
}

#[cfg(test)]
mod tests {
    use super::MultilinearPoly;
    use crate::MultilinearExtension;
    use p3_field::AbstractField;
    use p3_goldilocks::Goldilocks as F;

    fn f_abc() -> MultilinearPoly<F> {
        MultilinearPoly::new_from_vec(
            3,
            vec![0, 0, 0, 3, 0, 0, 2, 5]
                .into_iter()
                .map(F::from_canonical_u64)
                .collect(),
        )
    }

    #[test]
    fn test_mle_from_vec() {
        let _ = f_abc();
    }

    #[test]
    #[should_panic]
    fn test_mle_from_vec_var_mismatch() {
        let _ = MultilinearPoly::new_from_vec(
            3,
            vec![0, 0, 0, 3, 0, 0, 2]
                .into_iter()
                .map(F::from_canonical_u64)
                .collect(),
        );
    }

    #[test]
    fn test_partial_evaluation() {
        let poly = f_abc();
        let f_a = poly.partial_evaluate(&[F::from_canonical_u64(2), F::from_canonical_u64(3)]);
        assert_eq!(f_a.evaluations.len(), 2);
        assert_eq!(
            f_a.evaluations,
            &[F::from_canonical_u64(12), F::from_canonical_u64(21)]
        );
    }

    #[test]
    fn test_full_evaluation() {
        let poly = f_abc();
        let evaluation = poly.evaluate(&[
            F::from_canonical_u64(2),
            F::from_canonical_u64(3),
            F::from_canonical_u64(4),
        ]);
        assert_eq!(evaluation, F::from_canonical_u64(48));
    }
}
