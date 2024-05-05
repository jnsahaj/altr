use std::process;

pub mod cli;

fn main() {
    if let Err(e) = cli::run() {
        eprintln!("{e}");
        process::exit(1);
    }
}
