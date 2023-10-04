use crate::{fingerprint, Fingerprint, Seed, Words};
use crossbeam::channel::Sender;
use itertools::Itertools;
use rayon::prelude::*;

pub struct MultiCaretesianProduct {
    pub seed: Seed,
    pub words: Words,
    pub fingerprint: Fingerprint,
    pub progress: Option<Sender<()>>,
}

impl Default for MultiCaretesianProduct {
    fn default() -> Self {
        Self::new(Words::default(), Seed::default(), Fingerprint::default())
    }
}

impl MultiCaretesianProduct {
    pub fn new_from_env(words: String, seed: String, fingerprint: Option<String>) -> Self {
        let fingerprint = fingerprint.map(Into::into).unwrap_or_default();

        let words = Words::new_from_env(&words);
        Self::new(words, seed.into(), fingerprint)
    }

    pub fn new_with_words(words: Words) -> Self {
        Self::new(words, Seed::default(), Fingerprint::default())
    }

    pub fn new(words: Words, seed: Seed, fingerprint: Fingerprint) -> Self {
        Self {
            words,
            seed,
            fingerprint,
            progress: None,
        }
    }

    pub fn with_progress(mut self, progress: Sender<()>) -> Self {
        self.progress = Some(progress);
        self
    }

    pub fn set_progress(&mut self, progress: Sender<()>) {
        self.progress = Some(progress);
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

                if let Some(progress) = &self.progress {
                    progress.send(()).unwrap();
                }

                target_fingerprint == fingerprint
            })
            .map(|passphrase| passphrase.join(" "))
    }
}
