use std::io;
use clap::{Arg, App};

use colored::*;
extern crate clap;

mod recommenders;
mod report;
mod parser;

fn main() -> io::Result<()> {

    let matches = App::new("NameIt")
        .version("0.1.0")
        .author("Konstankino LLC")
        .about("Linter for your variables and constants.")
        .arg(Arg::with_name("FILE_PATH")
             .required(true)
             .takes_value(true)
             .index(1)
             .help("File to lint"))
        .get_matches();

    let file_name = matches.value_of("FILE_PATH").unwrap();

    let data = parser::Reader::read_file(file_name);

    match parser::Parser::parse_loc(data) {
        Ok(report_lines) => {
            for line in report_lines.iter() {
                println!("{:<3} {}", '\u{1F325}', line.title.white());
            }
        },
        Err(error) => println!("Unable to process. Error: {}", error),
    }

    Ok(())
}
