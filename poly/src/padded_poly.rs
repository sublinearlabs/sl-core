use std::marker::PhantomData;

use crate::MultilinearExtension;
use p3_field::{ExtensionField, Field};

// TODO: add documentation
enum BasePoly<E, T> {
    Eval(E),
    MLE(T),
}

// TODO: add documentation
pub struct PaddedPoly<F, E, T> {
    base_poly: BasePoly<E, T>,
    pad_count: usize,
    _marker: PhantomData<F>,
}

impl<F, E, T> PaddedPoly<F, E, T>
where
    F: Field,
    E: ExtensionField<F>,
    T: MultilinearExtension<F, E>,
{
    fn new(base_poly: T, pad_count: usize) -> Self {
        assert!(pad_count >= 1);
        assert!(base_poly.num_vars() >= 1);

        Self {
            base_poly: BasePoly::MLE(base_poly),
            pad_count,
            _marker: PhantomData,
        }
    }
}
