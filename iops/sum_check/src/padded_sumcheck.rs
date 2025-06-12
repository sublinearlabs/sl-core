use crate::sumcheckable::Sumcheckable;

struct PaddedSumcheck<S> {
    inner: S,
    pad_count: usize,
}
