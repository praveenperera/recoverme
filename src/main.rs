use clap::{Parser, Subcommand};
use combinator::{
    multi_cartesian_product::MultiCaretesianProduct, permutations::Permutations,
    progress_bar::ProgressBar,
};
use thousands::Separable;

#[derive(Parser)]
#[command(author, version, about, arg_required_else_help(true))]
pub struct Cli {
    #[command(subcommand)]
    command: crate::Command,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    #[command(visible_aliases = ["p", "perm"], subcommand)]
    Permutations(SubCommand),

    #[command(visible_aliases = ["mcp", "product", "cart"], subcommand)]
    MultiCartesianProduct(SubCommand),
}

#[derive(Subcommand, Debug, Clone)]
pub enum SubCommand {
    #[command(about = "Run the program")]
    Run {
        #[arg(short, long)]
        words: String,

        #[arg(short, long, env)]
        seed: String,

        #[arg(short, long, env)]
        fingerprint: String,

        #[arg(short, long, env)]
        number_of_words: Option<u16>,
    },

    #[command(about = "Count the number of options")]
    Count {
        #[arg(short, long)]
        words: String,

        #[arg(short, long, env)]
        seed: String,

        #[arg(short, long, env)]
        fingerprint: String,

        #[arg(short, long, env)]
        number_of_words: Option<u16>,
    },
}

fn main() {
    // initialize logging
    pretty_env_logger::init();

    // pretty errors
    color_eyre::install().unwrap();

    let cli = Cli::parse();

    match cli.command {
        Command::Permutations(SubCommand::Count {
            words,
            seed,
            fingerprint,
            number_of_words,
        }) => {
            let app = Permutations::new_from_env(words, seed, fingerprint, number_of_words);
            println!("{}", app.count().separate_with_commas());
        }
        Command::Permutations(SubCommand::Run {
            words,
            seed,
            fingerprint,
            number_of_words,
        }) => {
            let app = Permutations::new_from_env(words, seed, fingerprint, number_of_words);
            let count = app.count();
            let progress_bar = ProgressBar::new(count);

            let app = app.with_progress(progress_bar.sender.clone());
            progress_bar.listen();

            let message = if let Some(passphrase) = app.run() {
                format!("Passphrase FOUND!: {}", passphrase)
            } else {
                "No passphrase found".to_string()
            };

            println!("{message}");
            std::fs::write("passphrase.txt", message).unwrap();
        }

        Command::MultiCartesianProduct(SubCommand::Count {
            words,
            seed,
            fingerprint,
            ..
        }) => {
            let app = MultiCaretesianProduct::new_from_env(words, seed, fingerprint);
            println!("{}", app.count().separate_with_commas());
        }
        Command::MultiCartesianProduct(SubCommand::Run {
            words,
            seed,
            fingerprint,
            ..
        }) => {
            let app = MultiCaretesianProduct::new_from_env(words, seed, fingerprint);

            let count = app.count();
            let progress_bar = ProgressBar::new(count);

            let app = app.with_progress(progress_bar.sender.clone());
            progress_bar.listen();

            let message = if let Some(passphrase) = app.run() {
                format!("Passphrase FOUND!: {}", passphrase)
            } else {
                "No passphrase found".to_string()
            };

            println!("{message}");
            std::fs::write("passphrase.txt", message).unwrap();
        }
    }
}
