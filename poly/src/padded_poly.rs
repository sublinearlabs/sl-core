use std::marker::PhantomData;

use crate::{Fields, MultilinearExtension};
use p3_field::{ExtensionField, Field};

// TODO: add documentation
enum BasePoly<E, T> {
    Eval(E),
    MLE(T),
}

// brief thoughts on generics
// you are stating that you can parametize a structure with different types
// the compiler will then see all the invocation points and then help you
// duplicate your implementation based on those types
// hence generics are useful when you want the same piece of code to work
// with different types but want just one implementation
// and all of this happens at compile time
// you can also help the compiler perform some checks on the types
// i.e restrict the types that can make use of this code via trait bounds
// all types that can use this code must have implemented x, y, z
// bounds placed on a struct generic meaning if that struct is used as a field type
// for another struct, that bigger struct also has to enforce those bounds
// you cannot put everything into something only something into something
// but you can put something into everything
//
//
// an analysis on the truth of these things
// how can we make sense of it generically???
// it should hold a structure for sure then add padding to that structure
// what should be the type of this structure that it holds????

impl<E, T> BasePoly<E, T> {
    // TODO: add documentation
    fn as_mle(&self) -> Option<&T> {
        match self {
            Self::Eval(_) => None,
            Self::MLE(p) => Some(p),
        }
    }

    fn num_vars(&self) -> usize {
        todo!()
        //self.as_mle().map(|p| p.num_vars()).unwrap_or(0)
    }
}

// TODO: add documentation
// make reference to front loaded assignment
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

    fn eval(&self, points: &[Fields<F, E>]) -> Fields<F, E> {
        assert_eq!(self.num_vars(), points.len());
        // pull the amount needed then eval poly
        // mul that with the prod of the remaining, return that

        todo!()
    }
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
