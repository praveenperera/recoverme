use crate::{fingerprint, math, Fingerprint, Seed, Words, PASSPHRASE_LENGTH};
use crossbeam::channel::Sender;
use itertools::Itertools;
use rayon::prelude::*;

#[derive(Debug, Clone)]
pub struct Permutations {
    pub seed: Seed,
    pub words: Vec<String>,
    pub fingerprint: Fingerprint,
    pub number_of_words: u16,

    pub progress: Option<Sender<()>>,
}

impl Permutations {
    pub fn new_from_env(
        words: String,
        seed: String,
        fingerprint: String,
        number_of_words: Option<u16>,
    ) -> Self {
        let words = Words::new_from_env(&words);
        let number_of_words = number_of_words.unwrap_or(words.0.len() as u16);
        let fingerprint = Fingerprint::from(fingerprint);

        Self::new(words, seed.into(), fingerprint, number_of_words)
    }

    pub fn new_with_words(words: Words) -> Self {
        let number_of_words = words.0.len() as u16;

        Self::new(
            words,
            Seed::default(),
            Fingerprint::default(),
            number_of_words,
        )
    }

    pub fn new(words: Words, seed: Seed, fingerprint: Fingerprint, number_of_words: u16) -> Self {
        Self {
            words: words.0.into_iter().flatten().collect(),
            seed,
            fingerprint,
            number_of_words,

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
        math::permuations(self.words.len() as u128, PASSPHRASE_LENGTH as u128)
    }

    pub fn run(self) -> Option<String> {
        log::info!("Starting permutations: {}", self.count());
        let words_len = PASSPHRASE_LENGTH;
        let target_fingerprint: [u8; 4] = self.fingerprint.0;

        let seed = &self.seed.0;

        self.words
            .into_iter()
            .permutations(words_len)
            .par_bridge()
            .find_any(|passphrase| {
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

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions_sorted::assert_eq;

    // expensive test
    // TODO: use feature to exclude from regular testing
    #[test]
    fn find_with_no_combinations() {
        let words: Words = vec![
            vec!["benefit"],
            vec!["wife"],
            vec!["soccer"],
            vec!["rookie"],
            vec!["nation"],
            vec!["special"],
            vec!["child"],
        ]
        .into();

        let seed = "build since save grit begin key leisure similar royal diagram warfare execute laptop dress occur sword use soon above obtain beyond merry notable typical";
        let fingerprint: [u8; 4] = hex::decode("af849feb").unwrap()[..4].try_into().unwrap();
        let app = Permutations::new(words, seed.to_string().into(), fingerprint.into(), 7);

        assert_eq!(
            app.run(),
            Some("benefit wife soccer rookie nation special child".to_string())
        );
    }
}
