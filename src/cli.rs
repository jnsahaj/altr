use std::{
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader},
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

fn get_file_reader(path: &str) -> Result<impl BufRead, io::Error> {
    let file = OpenOptions::new().read(true).open(path)?;
    Ok(BufReader::new(file))
}

fn get_file_writer(path: &str) -> Result<File, io::Error> {
    let file = OpenOptions::new().write(true).open(path)?;
    Ok(file)
}

pub fn run() -> Result<(), clap::Error> {
    let cli = Cli::parse();
    dbg!(&cli);

    let mut task = Task::build(&cli.candidate, &cli.rename).unwrap();

    let mut records = task.generate_records(get_file_reader(&cli.file)?)?;
    let buf = task.process_records(&mut records, &mut get_file_reader(&cli.file)?);

    let mut writer = get_file_writer(&cli.file)?;

    writer.set_len(buf.len() as u64)?;

    Task::write(&mut writer, &buf)?;

    Ok(())
}
