use crate::{fingerprint, math, Fingerprint, Seed, Words};
use crossbeam::channel::Sender;
use itertools::Itertools;
use rayon::prelude::*;

#[derive(Debug, Clone)]
pub struct Permutations {
    pub seed: Seed,
    pub words: Vec<String>,
    pub fingerprint: Fingerprint,
    pub progress: Option<Sender<()>>,
}

impl Default for Permutations {
    fn default() -> Self {
        Self::new(Words::default(), Seed::default(), Fingerprint::default())
    }
}

impl Permutations {
    pub fn new_from_env(words: String, seed: String) -> Self {
        let words = Words::new_from_env(&words);
        Self::new(words, seed.into(), Fingerprint::default())
    }

    pub fn new_with_words(words: Words) -> Self {
        Self::new(words, Seed::default(), Fingerprint::default())
    }

    pub fn new(words: Words, seed: Seed, fingerprint: Fingerprint) -> Self {
        Self {
            words: words.0.into_iter().flatten().collect(),
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
        math::permuations(self.words.len() as u128, 7)
    }

    pub fn run(self) -> Option<String> {
        log::info!("Starting permutations: {}", self.count());
        let words_len = self.words.len();
        let target_fingerprint: [u8; 4] = self.fingerprint.0;

        let seed = &self.seed.0;

        self.words
            .into_iter()
            .permutations(words_len)
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
