use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, derive_more::From, derive_more::Deref)]
pub struct Words(Vec<Vec<String>>);

fn main() {
    let words_json = std::env::var("WORDS").unwrap();
    let words: Words = serde_json::from_str(&words_json).unwrap();

    let mut combos = Vec::new();
    let words_len = words.len();

    for prod in words.0.into_iter().multi_cartesian_product() {
        for p in prod.into_iter().permutations(words_len) {
            let combo = p.join(" ");
            combos.push(combo);
        }
    }

    println!("{}", combos.len());
}
