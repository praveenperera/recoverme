use std::sync::{Arc, Mutex};

use derive_more::{Deref, From};
use itertools::Itertools;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, From, Deref)]
pub struct Words(Vec<Vec<String>>);

impl From<Vec<Vec<&str>>> for Words {
    fn from(words: Vec<Vec<&str>>) -> Self {
        let words: Vec<Vec<String>> = words
            .into_iter()
            .map(|v| v.into_iter().map(|s| s.to_string()).collect())
            .collect();
        Words(words)
    }
}

#[derive(Debug, Clone)]
pub struct App {
    pub seed: String,
    pub words: Words,
    pub fingerprint: [u8; 4],
}

pub const FINGERPRINT: &str = "1cf84a2a";

impl Default for App {
    fn default() -> Self {
        let words_json = std::env::var("WORDS").unwrap();

        let words: Words = serde_json::from_str(&words_json).unwrap();
        let seed = std::env::var("SEED").unwrap();
        let target_fingerprint = hex::decode(FINGERPRINT).unwrap()[..4].try_into().unwrap();

        Self::new(words, seed, target_fingerprint)
    }
}

impl App {
    pub fn new(words: Words, seed: String, fingerprint: [u8; 4]) -> Self {
        Self {
            words,
            seed,
            fingerprint,
        }
    }

    pub fn run(self) -> Option<String> {
        let words_len = self.words.len();
        let target_fingerprint: [u8; 4] = self.fingerprint;

        let seed = &self.seed;
        let found = Arc::new(Mutex::new(None));

        self.words
            .0
            .into_iter()
            .multi_cartesian_product()
            .par_bridge()
            .for_each(|prod| {
                prod.into_iter()
                    .permutations(words_len)
                    .par_bridge()
                    .for_each(|passphrase| {
                        let passphrase_string = passphrase.join("");
                        let fingerprint =
                            combinator::fingerprint::create_fingerprint(seed, &passphrase_string)
                                .unwrap();

                        if target_fingerprint == fingerprint {
                            println!(
                                "Found fingerprint: {}",
                                hex::encode(target_fingerprint.clone())
                            );

                            println!("Passphrase: {}", passphrase.join(" "));
                            *found.lock().unwrap() = Some(passphrase.join(" "));
                        }
                    });
            });

        if let Some(passphrase) = found.lock().unwrap().clone() {
            return Some(passphrase);
        }

        None
    }
}

fn main() {
    // let combos = Arc::new(Mutex::new(0));
    // let combos_clone = combos.clone();

    // let (sender, receiver) = crossbeam::channel::bounded(1000);
    let app_thread = std::thread::spawn(move || {
        let app = App::default();
        app.run();
    });

    // let writer_thread = std::thread::spawn(move || {
    //     let file = std::fs::File::create("output.txt").unwrap();
    //     let mut writer = std::io::BufWriter::new(file);
    //
    //     for (passphrase, fingerprint) in receiver.iter() {
    //         writer.write_all(passphrase.as_bytes()).unwrap();
    //         writer.write_all(b"=").unwrap();
    //         writer.write_all(&fingerprint).unwrap();
    //         writer.write_all(b"\n").unwrap();
    //     }
    //
    //     writer.flush().unwrap();
    // });

    // let running = Arc::new(AtomicBool::new(true));
    // let r = running.clone();
    //
    // ctrlc::set_handler(move || {
    //     r.store(false, Ordering::SeqCst);
    // })
    // .expect("Error setting Ctrl-C handler");
    //
    // println!("Waiting for Ctrl-C...");
    // while running.load(Ordering::SeqCst) {}
    // println!("Got it! Exiting...");
    //
    // let combos = combos.lock().unwrap();
    // println!("Combinations checked: {}", combos);
    //
    app_thread.join().unwrap();
    println!("App thread joined");

    // writer_thread.join().unwrap();
}
