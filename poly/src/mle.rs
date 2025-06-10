use std::ops::{Add, Index, Mul, Sub};

use p3_field::{ExtensionField, Field};

use crate::{Fields, MultilinearExtension};

#[derive(Debug, Clone, PartialEq)]
pub struct MultilinearPoly<F: Field, E: ExtensionField<F>> {
    /// The evaluations of the boolean hypercube {0,1}^n_vars
    pub evaluations: Vec<Fields<F, E>>,
    /// Number of variables
    n_vars: usize,
}

impl<F: Field, E: ExtensionField<F>> MultilinearPoly<F, E> {
    /// Instantiates a `MultilinearPoly` from a vec of field elements
    pub fn new_from_vec(n_vars: usize, evaluations: Vec<Fields<F, E>>) -> Self {
        // assert that the number of variables matches the number of evaluations
        assert_eq!(1 << n_vars, evaluations.len());
        Self {
            evaluations,
            n_vars,
        }
    }

    /// Creates a Zero Multilinear poly
    pub fn zero(num_vars: usize) -> Self {
        Self::new_from_vec(num_vars, vec![Fields::Base(F::zero()); 1 << num_vars])
    }
}

impl<F: Field, E: ExtensionField<F>> MultilinearExtension for MultilinearPoly<F, E> {
    type Field = F;
    type Extension = E;

    /// Partially fixes variables in the `MultilinearPoly`
    /// Returns a new `MultilinearPoly` after fixed variables have
    /// been removed
    fn partial_evaluate(&self, points: &[Fields<F, E>]) -> Self {
        // ensure we don't have more points than variables
        assert!(points.len() <= self.n_vars);

        let mut new_evaluations = self.evaluations.clone();

        // for each partial point, fold the evaluations in half
        let mut mid_point = new_evaluations.len() / 2;
        for point in points {
            for i in 0..mid_point {
                let left = new_evaluations[i].to_extension_field();
                let right = new_evaluations[i + mid_point].to_extension_field();
                new_evaluations[i] = match point {
                    // if the evaluation point is in the boolean hypercube
                    // return result from table directly
                    a if a.to_extension_field().is_zero() => Fields::Extension(left),
                    a if a.to_extension_field().is_one() => Fields::Extension(right),

                    // linear interpolation
                    // (1-r) * left + r * right
                    // left - r.left + r.right
                    // left - r (left - right)
                    _ => Fields::Extension(left - point.to_extension_field() * (left - right)),
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
    fn evaluate(&self, points: &[Fields<F, E>]) -> Fields<F, E> {
        // ensure number of points exactly matches number of variables
        assert_eq!(self.n_vars, points.len());
        self.partial_evaluate(points).evaluations[0]
    }

    /// Polynomial max variable degree
    fn max_degree(&self) -> usize {
        1
    }

    /// Returns the sum of evaluations overr the boolean hypercube
    fn sum_over_hypercube(&self) -> Fields<F, E> {
        self.evaluations
            .iter()
            .fold(Fields::Base(F::zero()), |acc, curr| {
                Fields::Extension(acc.to_extension_field() + curr.to_extension_field())
            })
    }

    /// Number of variables in the `MultilinearPoly`
    fn num_vars(&self) -> usize {
        self.n_vars
    }

    /// Commit `MultilinearPoly` to transcript
    fn commit_to_transcript(&self, transcript: &mut transcript::Transcript<F, E>)
    where
        F: p3_field::PrimeField32,
    {
        for eval in &self.evaluations {
            match eval {
                Fields::Base(base_elem) => transcript.observe_base_element(&[*base_elem]),
                Fields::Extension(ext_elem) => transcript.observe_ext_element(&[*ext_elem]),
            }
        }
    }
}

impl<F: Field, E: ExtensionField<F>> Index<usize> for MultilinearPoly<F, E> {
    type Output = Fields<F, E>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.evaluations[index]
    }
}

impl<F: Field, E: ExtensionField<F>> From<Vec<E>> for MultilinearPoly<F, E> {
    fn from(evaluations: Vec<E>) -> Self {
        let n_vars = (evaluations.len() as f64).log2() as usize;
        let evaluations = evaluations
            .into_iter()
            .map(|eval| Fields::Extension(eval))
            .collect();
        MultilinearPoly {
            n_vars,
            evaluations,
        }
    }
}

impl<F: Field, E: ExtensionField<F>> From<&Vec<F>> for MultilinearPoly<F, E> {
    fn from(evaluations: &Vec<F>) -> Self {
        let n_vars = (evaluations.len() as f64).log2() as usize;
        let evaluations = evaluations
            .iter()
            .map(|&x| Fields::<F, E>::Base(x))
            .collect::<Vec<_>>();

        MultilinearPoly {
            n_vars,
            evaluations,
        }
    }
}

impl<F: Field, E: ExtensionField<F>> From<Vec<Fields<F, E>>> for MultilinearPoly<F, E> {
    fn from(evaluations: Vec<Fields<F, E>>) -> Self {
        let n_vars = (evaluations.len() as f64).log2() as usize;

        MultilinearPoly {
            n_vars,
            evaluations,
        }
    }
}

impl<F: Field, E: ExtensionField<F>> Add for MultilinearPoly<F, E> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        if self.n_vars != other.n_vars {
            panic!("Polynomials must have the same number of variables");
        }

        let mut new_evaluations = Vec::new();

        for i in 0..self.evaluations.len() {
            new_evaluations.push(self.evaluations[i] + other.evaluations[i]);
        }

        MultilinearPoly::from(new_evaluations)
    }
}

impl<F: Field, E: ExtensionField<F>> Sub for MultilinearPoly<F, E> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        if self.n_vars != other.n_vars {
            panic!("Polynomials must have the same number of variables");
        }

        let mut new_evaluations = Vec::new();

        for i in 0..self.evaluations.len() {
            new_evaluations.push(self.evaluations[i] - other.evaluations[i]);
        }

        MultilinearPoly::from(new_evaluations)
    }
}

impl<F: Field, E: ExtensionField<F>> Mul<Fields<F, E>> for MultilinearPoly<F, E> {
    type Output = Self;

    fn mul(self, other: Fields<F, E>) -> Self::Output {
        let evaluations = self
            .evaluations
            .iter()
            .map(|eval| *eval * other)
            .collect::<Vec<_>>();

        MultilinearPoly::from(evaluations)
    }
}

#[cfg(test)]
mod tests {
    use super::MultilinearPoly;
    use crate::{mle::Fields, MultilinearExtension};
    use p3_field::{extension::BinomialExtensionField, AbstractField};
    use p3_goldilocks::Goldilocks as F;

    type E = BinomialExtensionField<F, 2>;

    fn f_abc() -> MultilinearPoly<F, E> {
        MultilinearPoly::new_from_vec(
            3,
            vec![0, 0, 0, 3, 0, 0, 2, 5]
                .into_iter()
                .map(|val| Fields::Base(F::from_canonical_u64(val)))
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
                .map(|val| Fields::<F, E>::Base(F::from_canonical_u64(val)))
                .collect(),
        );
    }

    #[test]
    fn test_partial_evaluation() {
        let poly = f_abc();
        let f_a = poly.partial_evaluate(&[
            Fields::Base(F::from_canonical_u64(2)),
            Fields::Base(F::from_canonical_u64(3)),
        ]);
        assert_eq!(f_a.evaluations.len(), 2);
        assert_eq!(
            f_a.evaluations,
            &[
                Fields::Extension(E::from_canonical_u64(12)),
                Fields::Extension(E::from_canonical_u64(21))
            ]
        );
    }

    #[test]
    fn test_full_evaluation() {
        let poly = f_abc();
        let evaluation = poly.evaluate(&[
            Fields::Base(F::from_canonical_u64(2)),
            Fields::Base(F::from_canonical_u64(3)),
            Fields::Base(F::from_canonical_u64(4)),
        ]);
        assert_eq!(evaluation, Fields::Extension(E::from_canonical_u64(48)));
    }

    #[test]
    fn test_sum_over_boolean_hypercube() {
        let poly = f_abc();
        assert_eq!(
            poly.sum_over_hypercube(),
            Fields::Extension(E::from_canonical_u64(10))
        );
    }
}
