mod casing;
mod cli;
mod record;
mod task;
mod token;

const SEPARATOR: char = ',';

fn main() {
    let _ = cli::run();
}
