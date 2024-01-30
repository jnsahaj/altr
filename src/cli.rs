use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, Write},
};

use clap::Parser;

use crate::task::Task;

#[derive(Parser, Debug)]
struct Cli {
    candidate: String,
    rename_to: String,

    #[arg(short, long)]
    file: String,
}

fn get_file_reader(path: &str) -> impl BufRead {
    let file = OpenOptions::new().read(true).open(path).unwrap();

    BufReader::new(file)
}

fn get_file_writer(path: &str) -> impl Write {
    let file = OpenOptions::new().write(true).open(path).unwrap();
    file
}

pub fn run() -> Result<(), clap::Error> {
    let cli = Cli::parse();
    dbg!(&cli);

    let mut task = Task::new(&cli.candidate, &cli.rename_to);

    let mut records = task.generate_records(get_file_reader(&cli.file));

    task.process_records(
        &mut records,
        get_file_reader(&cli.file),
        get_file_writer(&cli.file),
    );

    Ok(())
}
