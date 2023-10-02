use std::sync::{Arc, Mutex};

use derive_more::{Deref, From};
use itertools::Itertools;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, From, Deref)]
pub struct Words(Vec<Vec<String>>);

fn main() {
    let words_json = std::env::var("WORDS").unwrap();
    let words: Words = serde_json::from_str(&words_json).unwrap();

    let combos = Arc::new(Mutex::new(Vec::new()));
    let words_len = words.len();

    words
        .0
        .into_iter()
        .multi_cartesian_product()
        .par_bridge()
        .for_each(|prod| {
            prod.into_iter()
                .permutations(words_len)
                .par_bridge()
                .for_each(|p| {
                    let combo = p.join(" ");
                    combos.lock().unwrap().push(combo);
                });
        });

    println!("{}", combos.lock().unwrap().len());
}
