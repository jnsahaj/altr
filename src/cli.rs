use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader},
};

use clap::Parser;

use crate::task::Task;

#[derive(Parser, Debug)]
struct Cli {
    candidate: String,
    rename: String,

    #[arg(short, long)]
    file: String,
}

fn get_file_reader(path: &str) -> impl BufRead {
    let file = OpenOptions::new().read(true).open(path).unwrap();
    BufReader::new(file)
}

fn get_file_writer(path: &str) -> File {
    let file = OpenOptions::new().write(true).open(path).unwrap();
    file
}

pub fn run() -> Result<(), clap::Error> {
    let cli = Cli::parse();
    dbg!(&cli);

    let mut task = Task::new(&cli.candidate, &cli.rename);

    let mut records = task.generate_records(get_file_reader(&cli.file));
    let buf = task.process_records(&mut records, &mut get_file_reader(&cli.file));

    let mut writer = get_file_writer(&cli.file);

    writer.set_len(buf.len().try_into().unwrap()).unwrap();

    Task::write(&mut writer, &buf);

    Ok(())
}
