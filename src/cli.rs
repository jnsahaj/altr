use std::fs::File;

use clap::Parser;

use crate::task::Task;

#[derive(Parser, Debug)]
struct Cli {
    candidate: String,
    rename_to: String,

    #[arg(short, long)]
    file: String,
}

pub fn run() -> Result<(), clap::Error> {
    let cli = Cli::parse();
    dbg!(&cli);

    let file = File::open(cli.file).unwrap();

    let mut task = Task::new(file, &cli.candidate, &cli.rename_to);

    let mut records = task.generate_records();
    task.process_records(&mut records);

    Ok(())
}
