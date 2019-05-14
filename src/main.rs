use std::{fs::File, io::{self, BufReader, prelude::*}};
use clap::{Arg, App};
use colored::*;

extern crate clap;
extern crate regex;

extern crate colored;

mod recommenders;
mod reports;

use self::recommenders::{VariableKind, Variable, name_validity::NameValidity};
use self::reports::{Report};

fn main() -> io::Result<()> {

    let matches = App::new("NameIT")
        .version("0.1.0")
        .author("Konstankino LLC")
        .about("Linter for your variables and constants.")
        .arg(Arg::with_name("LINE")
             .required(true)
             .takes_value(true)
             .index(1)
             .help("LOC to lint"))
        .get_matches();

    let let_re = regex::Regex::new(r"let +(\w+) *?= *?([[:punct:][:alnum:]]+)\.*?").unwrap();
    let var_re = regex::Regex::new(r"var +(\w+) *?= *?([[:punct:][:alnum:]]+)\.*?").unwrap();
    let undef_re = regex::Regex::new(r"(\w+) *?= *?([[:punct:][:alnum:]]+)\.*?").unwrap();

    let file_name = matches.value_of("LINE").unwrap();
    let f = File::open(file_name)?;
    let f = BufReader::new(f);

    for (linum, line) in f.lines().enumerate() {

        let text = line.unwrap().to_string();

        let mut variables: Box<Vec<Variable>> = Box::new(Vec::new());

        let _ = matches.value_of("LINE").unwrap();

        if text.len() < 4 {
            continue
        }

        for cap in let_re.captures_iter(text.as_str()) {
            let var = Variable::new(&cap[1], VariableKind::Let, &cap[2], linum);
            if !variables.contains(&var) {
                variables.push(var);
            }
        }
        for cap in var_re.captures_iter(text.as_str()) {
            let var = Variable::new(&cap[1], VariableKind::Var, &cap[2], linum);
            if !variables.contains(&var) {
                variables.push(var);
            }
        }

        for cap in undef_re.captures_iter(text.as_str()) {
            let var = Variable::new(&cap[1], VariableKind::Global, &cap[2], linum);
            if !variables.contains(&var) {
                variables.push(var);
            }
        }

        let _rep = Report::new(variables, vec![
            Box::new(NameValidity{}),
        ]);
        for line in _rep.generate().iter() {
            println!("{:>5} {:>20} {}", '\u{1F325}', line.variable_name.red(), line.title.white());
        }
    }

    Ok(())
}
