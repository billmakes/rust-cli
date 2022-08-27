use clap::{value_parser, Arg, Command};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Clone)]
pub struct Config {
    files: Vec<String>,
    bytes: Option<usize>,
    lines: usize,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("headr")
        .version("0.1.0")
        .author("Bill Hilbert <billmakes@protonmail.com>")
        .about("Rust head")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple_values(true)
                .default_value("-"),
        )
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .value_name("BYTES")
                .takes_value(true)
                .value_parser(value_parser!(usize))
                .conflicts_with("lines")
                .help("In bytes"),
        )
        .arg(
            Arg::new("lines")
                .short('n')
                .long("lines")
                .takes_value(true)
                .value_parser(value_parser!(usize))
                .help("Number of lines")
                .default_value("10"),
        )
        .get_matches();

    let files = matches.get_many("files").unwrap().cloned().collect();
    let bytes = matches.get_one("bytes").cloned();
    let lines = matches.get_one("lines").cloned().unwrap();

    Ok(Config {
        files,
        bytes,
        lines,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let num_files = config.files.len();
    for (file_num, filename) in config.files.iter().enumerate() {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(mut file) => {
                if num_files > 1 {
                    println!(
                        "{}==> {} <==",
                        if file_num > 0 { "\n" } else { "" },
                        &filename
                    );
                }

                if let Some(num_bytes) = config.bytes {
                    let mut handle = file.take(num_bytes as u64);
                    let mut buffer = vec![0; num_bytes];
                    let bytes_read = handle.read(&mut buffer)?;
                    print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                } else {
                    let mut line = String::new();
                    for _ in 0..config.lines {
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{}", line);
                        line.clear();
                    }
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
