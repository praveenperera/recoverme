use crate::{fingerprint, Fingerprint, Seed, Words};
use itertools::Itertools;
use rayon::prelude::*;

pub struct MultiCaretesianProduct {
    pub seed: Seed,
    pub words: Words,
    pub fingerprint: Fingerprint,
}

impl Default for MultiCaretesianProduct {
    fn default() -> Self {
        Self::new(Words::default(), Seed::default(), Fingerprint::default())
    }
}

impl MultiCaretesianProduct {
    pub fn new_from_env(words: Option<String>) -> Self {
        if let Some(words) = words {
            let words = Words::new_from_env(&words);
            Self::new_with_words(words)
        } else {
            Self::default()
        }
    }

    pub fn new_with_words(words: Words) -> Self {
        Self::new(words, Seed::default(), Fingerprint::default())
    }

    pub fn new(words: Words, seed: Seed, fingerprint: Fingerprint) -> Self {
        Self {
            words,
            seed,
            fingerprint,
        }
    }

    pub fn count(&self) -> u128 {
        self.words.0.iter().multi_cartesian_product().count() as u128
    }

    pub fn run(self) -> Option<String> {
        log::info!("Starting multi cartesian product: {}", self.count());
        let target_fingerprint: [u8; 4] = self.fingerprint.0;

        let seed = &self.seed.0;
        self.words
            .0
            .into_iter()
            .multi_cartesian_product()
            .par_bridge()
            .find_first(|passphrase| {
                let passphrase_string = passphrase.join("");
                let fingerprint = fingerprint::create_fingerprint(seed, passphrase_string).unwrap();

                target_fingerprint == fingerprint
            })
            .map(|passphrase| passphrase.join(" "))
    }
}
