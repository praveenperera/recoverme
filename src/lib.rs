use derive_more::{Deref, From};
use serde::{Deserialize, Serialize};

pub mod fingerprint;
pub mod math;
pub mod multi_cartesian_product;
pub mod permutations;

pub const PASSPHRASE_LENGTH: usize = 7;

#[derive(Debug, Clone, Serialize, Deserialize, From, Deref)]
pub struct Words(Vec<Vec<String>>);

#[derive(Debug, Clone, Serialize, Deserialize, From, Deref)]
pub struct Seed(String);

#[derive(Debug, Clone, Serialize, Deserialize, From, Deref)]
pub struct Fingerprint([u8; 4]);

impl Words {
    pub fn new_from_env(env: &str) -> Self {
        let words_json = std::env::var(env).unwrap();
        serde_json::from_str(&words_json).unwrap()
    }
}

impl Default for Words {
    fn default() -> Self {
        Self::new_from_env("WORDS")
    }
}

impl Default for Seed {
    fn default() -> Self {
        let seed = std::env::var("SEED").unwrap();
        Self(seed)
    }
}

impl Default for Fingerprint {
    fn default() -> Self {
        let fingerprint = std::env::var("FINGERPRINT").expect("FINGERPRINT env var not found");
        let target_fingerprint = hex::decode(fingerprint).expect("FINGERPRINT was not valid hex")
            [..4]
            .try_into()
            .unwrap();

        Self(target_fingerprint)
    }
}

impl From<Vec<Vec<&str>>> for Words {
    fn from(words: Vec<Vec<&str>>) -> Self {
        let words: Vec<Vec<String>> = words
            .into_iter()
            .map(|v| v.into_iter().map(|s| s.to_string()).collect())
            .collect();
        Words(words)
    }
}
