//! This module contains the implementation of the virtual polynomial.
//! A virtual polynomials is a Vector of MLEs having a combination relationship.
use p3_field::Field;
use crate::mle::MLE;

// TODO: added derived macros // #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VPoly<F: Field> {
    pub mles: Vec<MLE<F>>,
}
