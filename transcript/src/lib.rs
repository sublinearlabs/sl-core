use std::marker::PhantomData;

use fields::Fields;
use p3_challenger::{CanObserve, FieldChallenger, HashChallenger, SerializingChallenger32};
use p3_field::{ExtensionField, Field, PrimeField32};
use p3_keccak::Keccak256Hash;

pub struct Transcript<F: Field, E: ExtensionField<F>> {
    _marker: PhantomData<(F, E)>,
    challenger: SerializingChallenger32<F, HashChallenger<u8, Keccak256Hash, 32>>,
}

impl<F: Field + PrimeField32, E: ExtensionField<F>> Transcript<F, E> {
    // Instantiate a transcript with a challenger
    pub fn init() -> Self {
        Self {
            _marker: PhantomData,
            challenger: SerializingChallenger32::new(HashChallenger::new(vec![], Keccak256Hash)),
        }
    }

    // Absorbs a byte array to the transcript
    pub fn observe(&mut self, vals: &[Fields<F, E>]) {
        for val in vals {
            match val {
                Fields::Base(_) => self
                    .observe_base_element(&[val.to_base_field().expect("confirmed base element")]),
                Fields::Extension(_) => self.observe_ext_element(&[val.to_extension_field()]),
            }
        }
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
    use p3_field::{extension::BinomialExtensionField, AbstractExtensionField, AbstractField};
    use p3_mersenne_31::Mersenne31;

    use crate::Transcript;

    #[test]
    fn test_transcript_initialization() {
        let mut transcript = Transcript::init();

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
