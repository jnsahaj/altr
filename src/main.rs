use std::process;

mod casing;
mod cli;
mod record;
mod task;
mod token;

const SEPARATOR: char = ',';

fn main() {
    if let Err(e) = cli::run() {
        eprintln!("{e}");
        process::exit(1);
    }
}
