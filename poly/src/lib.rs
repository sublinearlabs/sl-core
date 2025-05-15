use std::ops::{Add, Mul};

use p3_field::{ExtensionField, Field, PrimeField32};

pub mod mle;
pub mod utils;
pub mod vpoly;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Fields<F: Field, E: ExtensionField<F>> {
    Base(F),
    Extension(E),
}

impl<F: Field, E: ExtensionField<F>> Fields<F, E> {
    pub fn to_base_field(&self) -> Option<F> {
        match self {
            Fields::Base(val) => Some(*val),
            Fields::Extension(_) => panic!("Cant convert extension field to base field"),
        }
    }

    pub fn to_extension_field(&self) -> E {
        match self {
            Fields::Base(val) => E::from_base(*val),
            Fields::Extension(val) => *val,
        }
    }

    pub fn is_base_field(&self) -> bool {
        matches!(self, Fields::Base(_))
    }

    pub fn from_u32_vec(values: Vec<u32>) -> Vec<Self> {
        values
            .into_iter()
            .map(|val| Fields::Base(F::from_canonical_u32(val)))
            .collect()
    }

    pub fn from_u32(value: u32) -> Self {
        Fields::Base(F::from_canonical_u32(value))
    }
}

impl<F: Field, E: ExtensionField<F>> Add for Fields<F, E> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Fields::Base(lhs) => match rhs {
                Fields::Base(rhs_inner) => Fields::Base(lhs + rhs_inner),
                Fields::Extension(_) => panic!("mis-matched type"),
                // Fields::Extension(rhs_inner) => Fields::Extension(rhs_inner + lhs),
            },
            Fields::Extension(lhs) => match rhs {
                Fields::Base(_) => panic!("mis-matched type"),
                Fields::Extension(rhs_inner) => Fields::Extension(lhs + rhs_inner),
                // Fields::Extension(rhs_inner) => Fields::Extension(lhs + rhs_inner),
            },
        }
    }
}

impl<F: Field, E: ExtensionField<F>> Mul for Fields<F, E> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Fields::Base(lhs) => match rhs {
                Fields::Base(rhs_inner) => Fields::Base(lhs * rhs_inner),
                Fields::Extension(_) => panic!("mis-matched type"),
                // Fields::Extension(rhs_inner) => Fields::Extension(rhs_inner * lhs),
            },
            Fields::Extension(lhs) => match rhs {
                // Fields::Base(rhs_inner) => Fields::Extension(lhs * rhs_inner),
                Fields::Base(_) => panic!("mis-matched type"),
                Fields::Extension(rhs_inner) => Fields::Extension(lhs * rhs_inner),
            },
        }
    }
}

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

#[cfg(test)]
mod tests {
    use p3_field::{AbstractExtensionField, extension::BinomialExtensionField};

    use p3_mersenne_31::Mersenne31;

    use crate::Fields;

    type F = Mersenne31;

    type E = BinomialExtensionField<F, 3>;

    #[test]
    fn test_base_and_base_fields_addition() {
        let rhs_base_field_element = Fields::<F, E>::Base(F::new(2));

        let lhs_base_field_element = Fields::Base(F::new(2));

        let res = rhs_base_field_element + lhs_base_field_element;

        let expected = Fields::Base(F::new(4));

        assert_eq!(res, expected);
    }

    #[test]
    fn test_extension_and_extension_fields_addition() {
        let lhs_ext_field_element = Fields::<F, E>::Extension(E::from_base(F::new(5)));

        let rhs_ext_field_element = Fields::<F, E>::Extension(E::from_base(F::new(5)));

        let res = lhs_ext_field_element + rhs_ext_field_element;

        let expected = Fields::Extension(E::from_base(F::new(10)));

        assert_eq!(res, expected);
    }

    #[test]
    fn test_extension_and_base_fields_addition() {
        let ext_field_element = Fields::<F, E>::Extension(E::from_base(F::new(5)));

        let base_field_element = Fields::Base(F::new(2));

        let res = ext_field_element + base_field_element;

        let expected = Fields::Extension(E::from_base(F::new(7)));

        assert_eq!(res, expected);
    }

    #[test]
    fn test_base_and_base_fields_multiplication() {
        let rhs_base_field_element = Fields::<F, E>::Base(F::new(2));

        let lhs_base_field_element = Fields::Base(F::new(3));

        let res = rhs_base_field_element * lhs_base_field_element;

        let expected = Fields::Base(F::new(6));

        assert_eq!(res, expected);
    }

    #[test]
    fn test_extension_and_extension_fields_multiplication() {
        let lhs_ext_field_element = Fields::<F, E>::Extension(E::from_base(F::new(5)));

        let rhs_ext_field_element = Fields::<F, E>::Extension(E::from_base(F::new(5)));

        let res = lhs_ext_field_element * rhs_ext_field_element;

        let expected = Fields::Extension(E::from_base(F::new(25)));

        assert_eq!(res, expected);
    }

    #[test]
    fn test_extension_and_base_fields_multiplication() {
        let ext_field_element = Fields::<F, E>::Extension(E::from_base(F::new(5)));

        let base_field_element = Fields::Base(F::new(2));

        let res = ext_field_element * base_field_element;

        let expected = Fields::Extension(E::from_base(F::new(10)));

        assert_eq!(res, expected);
    }
}
