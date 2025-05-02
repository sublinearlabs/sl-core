use std::marker::PhantomData;

use p3_challenger::FieldChallenger;
use p3_field::{ExtensionField, Field};

pub struct Transcript<F: Field, E: ExtensionField<F>, FC: FieldChallenger<F>> {
    _marker: PhantomData<(F, E)>,
    challenger: FC,
}

impl<F: Field, E: ExtensionField<F>, FC: FieldChallenger<F>> Transcript<F, E, FC> {
    // Instantiate transcript
    pub fn init() -> Self {
        todo!()
    }

    // Instantiate a transcript with a challenger
    pub fn init_with_challenger(challenger: FC) -> Self {
        Self {
            _marker: PhantomData,
            challenger,
        }
    }

    // Absorbs a byte array to the transcript
    pub fn observe(&self, _message: &[u8]) {
        todo!()
    }

    // Absorbs an element from the base field to the transcript
    pub fn observe_base_element(&mut self, vals: &[F]) {
        for val in vals {
            self.challenger.observe(*val);
        }
    }

    // Absorbs elements from the extension field to the transcript
    pub fn observe_ext_element(&mut self, vals: &[E]) {
        for val in vals {
            self.challenger.observe_ext_element(*val);
        }
    }

    // Samples a random challenge in the extension field
    pub fn sample_challenge(&mut self) -> E {
        self.challenger.sample_ext_element()
    }

    // Samples n element from the extension field
    pub fn sample_n_challenges(&mut self, n: usize) -> Vec<E> {
        let mut res = Vec::with_capacity(n);

        for _ in 0..n {
            res.push(self.challenger.sample_ext_element());
        }

        res
    }
}

#[cfg(test)]
pub mod tests {
    use p3_challenger::{HashChallenger, SerializingChallenger32};
    use p3_field::{extension::BinomialExtensionField, AbstractExtensionField, AbstractField};
    use p3_keccak::Keccak256Hash;
    use p3_mersenne_31::Mersenne31;

    use crate::Transcript;

    #[test]
    fn test_transcript_initialization() {
        let challenger = SerializingChallenger32::new(HashChallenger::new(vec![], Keccak256Hash));

        let mut transcript = Transcript::init_with_challenger(challenger);

        let base_field_element = Mersenne31::from_canonical_u32(51);

        transcript.observe_base_element(&[base_field_element]);

        let challenge = transcript.sample_challenge();

        transcript.observe_ext_element(&[challenge]);

        let extension_field_element: BinomialExtensionField<Mersenne31, 3> =
            AbstractExtensionField::from_base(Mersenne31::new(5));

        transcript.observe_ext_element(&[extension_field_element]);

        let challenge = transcript.sample_challenge();

        transcript.observe_ext_element(&[challenge]);

        let challenge = transcript.sample_n_challenges(6);

        transcript.observe_ext_element(&challenge);
    }
}
