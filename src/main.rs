extern crate clap;

use std::fs;

use clap::{Arg, Command};

mod error_formatting;
mod input;
mod scanner;

const INPUT_STR: &str = "INPUT";
const SHOW_TOKENS_STR: &str = "tokens";

fn get_input(matches: &clap::ArgMatches) -> Option<input::Input> {
    if let Some(input_file) = matches.value_of(INPUT_STR) {
        println!("Reading from file {}", input_file);
        match fs::read_to_string(input_file) {
            Ok(input) => {
                return Some(input::Input {
                    source: input::Source::File(input_file.to_string()),
                    content: input,
                });
            }
            Err(err) => {
                println!("Error reading {}: {}", input_file, err);
                std::process::exit(-1);
            }
        }
    }
    None
}
fn main() {
    let matches = Command::new("samlox")
        .version("0.1.0")
        .about("lox language interpreter")
        .author("samuel thien")
        .arg(
            Arg::with_name(INPUT_STR)
                .help("sets input file to use")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::with_name(SHOW_TOKENS_STR)
                .long("--show-tokens")
                .takes_value(false)
                .help("show the token stream"),
        )
        .get_matches();
    println!("matches {:?}", matches);
    if let Some(input) = get_input(&matches) {
        if matches.is_present(SHOW_TOKENS_STR) {
            match scanner::scan_tokens(input.content.clone()) {
                Ok(tokens) => {
                    if matches.is_present(SHOW_TOKENS_STR) {
                        println!("{:#?}", tokens);
                        std::process::exit(0);
                    }
                }
                Err(errors) => {
                    error_formatting::format_lexical_error(&errors, &input);
                    std::process::exit(-1);
                }
            }
        }
    }
    print!("hello world")
}
