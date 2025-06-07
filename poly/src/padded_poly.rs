use std::marker::PhantomData;

use crate::MultilinearExtension;
use p3_field::{ExtensionField, Field};

// TODO: add documentation
enum BasePoly<E, T> {
    Eval(E),
    MLE(T),
}

impl<E, T> BasePoly<E, T> {
    fn num_vars<F>(&self) -> usize
    where
        F: Field,
        E: ExtensionField<F>,
        T: MultilinearExtension<F, E>,
    {
        match self {
            Self::Eval(_) => 0,
            Self::MLE(poly) => poly.num_vars(),
        }
    }
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
    // TODO: add documentation
    fn new(base_poly: T, pad_count: usize) -> Self {
        assert!(pad_count >= 1);
        assert!(base_poly.num_vars() >= 1);

        Self {
            base_poly: BasePoly::MLE(base_poly),
            pad_count,
            _marker: PhantomData,
        }
    }

    // TODO: add documentation
    fn num_vars(&self) -> usize {
        self.base_poly.num_vars() + self.pad_count
    }

    //fn eval(&self, points: &[Fields<F, E>]) -> Fields<F, E> {
    //    assert_eq!(self.num_vars)
    //
    //}
}

// how do I test this??
// I need a polynomial that I can pad
#[cfg(test)]
mod tests {
    use crate::padded_poly::{self, PaddedPoly};
    use crate::Fields;
    use crate::{mle::MultilinearPoly, MultilinearExtension};
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
    fn test_n_vars() {
        let p = f_abc();
        assert_eq!(p.num_vars(), 3);

        let padded_poly = PaddedPoly::new(p, 4);
        assert_eq!(padded_poly.num_vars(), 7);
    }
}
