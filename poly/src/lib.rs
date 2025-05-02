use p3_challenger::FieldChallenger;
use p3_field::{ExtensionField, Field};

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
        match self {
            Fields::Base(_) => true,
            _ => false,
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
    fn commit_to_transcript<FC: FieldChallenger<F>>(
        &self,
        transcript: &mut transcript::Transcript<F, E, FC>,
    );
}
