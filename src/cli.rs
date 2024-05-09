use std::{
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, Read, Write},
};

use altr::task::Task;
use altr::Result;

use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    candidate: String,
    rename: String,

    #[arg(short = 'f', long, default_value = "-")]
    input: String,

    #[arg(short, long)]
    output: Option<String>,
}

fn get_file_reader(path: &str) -> Result<impl BufRead> {
    let file = OpenOptions::new().read(true).open(path)?;
    Ok(BufReader::new(file))
}

fn get_file_writer(path: &str) -> Result<File> {
    let file = OpenOptions::new().write(true).open(path)?;
    Ok(file)
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    let mut buf = String::new();

    let _ = match cli.input.as_ref() {
        "-" => io::stdin().read_to_string(&mut buf),
        _ => get_file_reader(&cli.input)?.read_to_string(&mut buf),
    };

    let mut task = Task::build(&cli.candidate, &cli.rename, &buf)?;

    let mut records = task.generate_records();
    let (processed_buf, _) = task.process_records(&mut records);

    let output = cli.output.unwrap_or(cli.input);

    match output.as_ref() {
        "-" => {
            io::stdout().write_all(processed_buf.as_bytes())?;
        }
        _ => {
            let mut writer = get_file_writer(&output)?;
            writer.set_len(processed_buf.len() as u64)?;
            writer.write_all(processed_buf.as_bytes())?;
        }
    };

    Ok(())
}
