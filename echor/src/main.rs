use clap::{value_parser, Arg, Command};

fn main() {
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("Bill Hilbert <billmakes@protonmail.com>")
        .about("Rust echo")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .value_parser(value_parser!(String))
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Do not print newline")
                .takes_value(false),
        )
        .get_matches();

    let text: Vec<String> = matches
        .get_many("text")
        .expect("`text` is required")
        .cloned()
        .collect();
    let omit_newline = matches.contains_id("omit_newline");
    print! {"{}{}", text.join(" "), if omit_newline {""} else {"\n"}};
}
