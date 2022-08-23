use clap::{Arg, Command};
use std::error::Error;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("catr")
        .version("0.1.0")
        .author("Bill Hilbert <billmakes@protonmail.com")
        .about("Rust cat")
        .arg(
            Arg::new("files")
                .value_name("FILES")
                .help("Input files")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::new("number_lines")
                .short('n')
                .help("Include line numbers in output")
                .takes_value(false),
        )
        .arg(
            Arg::new("number_nonblank_lines")
                .short('b')
                .help("Include line numbers for every non-blank in output")
                .takes_value(false),
        )
        .get_matches();

    let files = matches
        .get_many("files")
        .expect("files required")
        .cloned()
        .collect();
    let number_lines = matches.contains_id("number_lines");
    let number_nonblank_lines = matches.contains_id("number_nonblank_lines");

    Ok(Config {
        files,
        number_lines,
        number_nonblank_lines,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);
    println!("Hello, world!");
    Ok(())
}
