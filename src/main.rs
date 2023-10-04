use clap::{Parser, Subcommand};
use combinator::{multi_cartesian_product::MultiCaretesianProduct, permutations::Permutations};
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
    },

    #[command(about = "Count the number of options")]
    Count {
        #[arg(short, long)]
        words: String,

        #[arg(short, long, env)]
        seed: String,
    },
}

fn main() {
    // initialize logging
    pretty_env_logger::init();

    // pretty errors
    color_eyre::install().unwrap();

    let cli = Cli::parse();

    match cli.command {
        Command::Permutations(SubCommand::Count { words, seed }) => {
            let app = Permutations::new_from_env(words, seed);

            println!("{}", app.count().separate_with_commas());
        }
        Command::Permutations(SubCommand::Run { words, seed }) => {
            let app = Permutations::new_from_env(words, seed);

            if let Some(passphrase) = app.run() {
                println!("Passphrase FOUND!: {}", passphrase);
            } else {
                println!("No passphrase found");
            }
        }

        Command::MultiCartesianProduct(SubCommand::Count { words, seed }) => {
            let app = MultiCaretesianProduct::new_from_env(words, seed);
            println!("{}", app.count().separate_with_commas());
        }
        Command::MultiCartesianProduct(SubCommand::Run { words, seed }) => {
            let app = MultiCaretesianProduct::new_from_env(words, seed);

            if let Some(passphrase) = app.run() {
                println!("Passphrase FOUND!: {}", passphrase);
            } else {
                println!("No passphrase found");
            }
        }
    }
}
