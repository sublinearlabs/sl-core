use crate::MultilinearExtension;

// TODO: add documentation
// TODO: also document each element
pub struct PaddedPoly<T: MultilinearExtension> {
    base_poly: T,
    n_vars: usize,
}

impl<T: MultilinearExtension> PaddedPoly<T> {
    fn new(base_poly: T, n_vars: usize) -> Self {
        // ensure that the setup actually requires padding
        assert!(n_vars > base_poly.num_vars());
        Self { base_poly, n_vars }
    }
}
