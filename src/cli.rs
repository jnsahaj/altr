use std::{
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, Read, Write},
};

use clap::Parser;

use crate::task::Task;

#[derive(Parser, Debug)]
struct Cli {
    candidate: String,
    rename: String,

    #[arg(short, long, default_value = "-")]
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

    let mut buf = String::new();

    let _ = match cli.file.as_ref() {
        "-" => io::stdin().read_to_string(&mut buf),
        _ => get_file_reader(&cli.file)?.read_to_string(&mut buf),
    };

    let mut task = Task::build(&cli.candidate, &cli.rename, &buf).unwrap();

    let mut records = task.generate_records()?;
    let processed_buf = task.process_records(&mut records);

    match cli.file.as_ref() {
        "-" => {
            io::stdout().write_all(processed_buf.as_bytes())?;
        }
        _ => {
            let mut writer = get_file_writer(&cli.file)?;
            writer.set_len(processed_buf.len() as u64)?;
            writer.write_all(processed_buf.as_bytes())?;
        }
    };

    Ok(())
}
