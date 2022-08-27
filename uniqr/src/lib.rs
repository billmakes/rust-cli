use clap::{Arg, Command};
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Write},
};
type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    in_file: String,
    out_file: Option<String>,
    count: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("uniqr")
        .version("0.1.0")
        .author("Bill Hilbert <billmakes@protonmail.com>")
        .about("Rust uniq")
        .arg(
            Arg::new("input")
                .value_name("INPUT")
                .help("Input file")
                .default_value("-"),
        )
        .arg(Arg::new("output").value_name("OUTPUT").help("Output file"))
        .arg(
            Arg::new("count")
                .short('c')
                .help("preflix lines by the number of occurences")
                .long("count")
                .takes_value(false),
        )
        .get_matches();

    let in_file = matches.get_one("input").cloned().unwrap();
    let out_file = matches.get_one("output").cloned();
    let count = matches.contains_id("count");
    Ok(Config {
        in_file,
        out_file,
        count,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let mut file = open(&config.in_file).map_err(|e| format!("{}: {}", config.in_file, e))?;
    let mut out_file: Box<dyn Write> = match &config.out_file {
        Some(out_name) => Box::new(File::create(out_name)?),
        _ => Box::new(io::stdout()),
    };

    let mut print = |count: u64, text: &str| -> MyResult<()> {
        if count > 0 {
            if config.count {
                write!(out_file, "{:>4} {}", count, text)?;
            } else {
                write!(out_file, "{}", text)?;
            }
        };
        Ok(())
    };
    let mut line = String::new();
    let mut previous = String::new();
    let mut count: u64 = 0;
    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        if line.trim_end() != previous.trim_end() {
            print(count, &previous)?;
            previous = line.clone();
            count = 0;
        }
        count += 1;
        line.clear();
    }
    print(count, &previous)?;
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
