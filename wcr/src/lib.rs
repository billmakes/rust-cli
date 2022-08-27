use clap::{value_parser, Arg, Command};
use std::error::Error;
use std::fmt::Write;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("headr")
        .version("0.1.0")
        .author("Bill Hilbert <billmakes@protonmail.com>")
        .about("Rust wc")
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
                .value_parser(value_parser!(bool))
                .takes_value(false)
                .help("Show byte count"),
        )
        .arg(
            Arg::new("chars")
                .short('m')
                .long("chars")
                .value_parser(value_parser!(bool))
                .takes_value(false)
                .conflicts_with("bytes")
                .help("Show character count"),
        )
        .arg(
            Arg::new("words")
                .short('w')
                .long("words")
                .value_parser(value_parser!(bool))
                .takes_value(false)
                .help("Show word count"),
        )
        .arg(
            Arg::new("lines")
                .short('l')
                .long("lines")
                .value_parser(value_parser!(bool))
                .takes_value(false)
                .help("Show line count"),
        )
        .get_matches();

    let files: Vec<String> = matches.get_many("files").unwrap().cloned().collect();
    let mut lines = matches.contains_id("lines");
    let mut words = matches.contains_id("words");
    let mut bytes = matches.contains_id("bytes");
    let chars = matches.contains_id("chars");

    if [lines, words, bytes, chars].iter().all(|v| v == &false) {
        lines = true;
        words = true;
        bytes = true;
    }

    Ok(Config {
        files,
        lines,
        words,
        bytes,
        chars,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_bytes = 0;
    let mut total_chars = 0;

    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                if let Ok(info) = count(file) {
                    println!(
                        "{}{}",
                        format_fields(
                            format_field,
                            vec![
                                (info.num_lines, config.lines),
                                (info.num_words, config.words),
                                (info.num_bytes, config.bytes),
                                (info.num_chars, config.chars),
                            ],
                        ),
                        if filename == "-" {
                            "".to_string()
                        } else {
                            format!(" {}", filename)
                        }
                    );
                    total_lines += info.num_lines;
                    total_words += info.num_words;
                    total_bytes += info.num_bytes;
                    total_chars += info.num_chars;
                }
            }
        }
    }
    if config.files.len() > 1 {
        println!(
            "{} total",
            format_fields(
                format_field,
                vec![
                    (total_lines, config.lines),
                    (total_words, config.words),
                    (total_bytes, config.bytes),
                    (total_chars, config.chars),
                ],
            )
        );
    }
    Ok(())
}

fn format_fields<T>(f: T, args: Vec<(usize, bool)>) -> String
where
    T: Fn(usize, bool) -> String,
{
    let mut result_str = String::new();
    for arg in args {
        write!(result_str, "{}", f(arg.0, arg.1)).unwrap();
    }
    result_str
}

fn format_field(value: usize, show: bool) -> String {
    if show {
        format!("{:>8}", value)
    } else {
        "".to_string()
    }
}

pub fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;
    let mut line = String::new();
    loop {
        let line_bytes = file.read_line(&mut line)?;
        if line_bytes == 0 {
            break;
        }
        num_bytes += line_bytes;
        num_lines += 1;
        num_words += line.split_whitespace().count();
        num_chars += line.chars().count();
        line.clear();
    }
    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

#[cfg(test)]
mod tests {
    use super::{count, format_field, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }

    #[test]
    fn test_format_field() {
        assert_eq!(format_field(1, false), "");
        assert_eq!(format_field(3, true), "       3");
        assert_eq!(format_field(10, true), "      10");
    }
}
