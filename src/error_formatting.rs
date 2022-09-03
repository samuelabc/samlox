use crate::input;
use crate::scanner;
use colored::*;

fn format_input(input: &input::Input, line: usize, col: i64) {
    eprintln!(
        "in {}, at line {}, column {}:",
        match &input.source {
            input::Source::Literal => "<command-line input>",
            input::Source::File(filename) => filename,
        },
        line,
        col
    );
    eprintln!("{}", input.content.lines().nth(line - 1).unwrap());
    eprint!("{:~<1$}", "".blue().bold(), col as usize);
    eprintln!("{}", "^".blue().bold());
}

pub fn format_lexical_error(errors: &Vec<scanner::Error>, input: &input::Input) {
    for error in errors {
        eprintln!(
            "loxi: {}: {}",
            "lexical error".red().bold(),
            error.message.white().bold(),
        );
        format_input(input, error.line, error.col);
    }
}
