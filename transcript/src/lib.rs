use p3_challenger::FieldChallenger;
use p3_field::{ExtensionField, Field};

pub struct Transcript<F: Field, E: ExtensionField<F>, FC: FieldChallenger<F>> {
    base_field: F,
    extension_field: E,
    challenger: FC,
}

impl<F: Field, E: ExtensionField<F>, FC: FieldChallenger<F>> Transcript<F, E, FC> {
    // Instantiate a transcript
    pub fn instantiate(challenger: FC) -> Self {
        Self {
            base_field: (),
            extension_field: (),
            challenger,
        }
    }

    // Absorbs an element from the base field to the transcript
    pub fn observe_base_element(&mut self, vals: &[F]) {
        for val in vals {
            self.observe_base_element(&[*val]);
        }
    }

    // Absorbs elements from the extension field to the transcript
    pub fn observe_ext_element(&mut self, vals: &[E]) {
        for val in vals {
            self.observe_ext_element(&[*val]);
        }
    }

    // Samples n element from the transcript
    // Sampled elements are always from the extension field
    pub fn sample_n_element(&mut self, n: u64) -> Vec<E> {
        let mut res = vec![];

        for _ in 0..n {
            res.push(self.challenger.sample_ext_element());
        }

        res
    }
}
