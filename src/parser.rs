use std::{fs::File, io::{self, BufReader, prelude::*}};

use crate::recommenders::{VariableKind, Variable, name_validity::NameValidity, ReportLine};
use crate::reports::{Report};

extern crate regex;

pub struct Parser {}

type ReportLineHeap = Box<Vec<ReportLine>>;
type VariablesHeap = Box<Vec<Variable>>;

impl Parser {

    pub fn parse(file_name: &str) -> io::Result<(ReportLineHeap)> {

        let let_re = regex::Regex::new(r"let +(\w+) *?= *?([[:punct:][:alnum:]]+)\.*?").unwrap();
        let var_re = regex::Regex::new(r"var +(\w+) *?= *?([[:punct:][:alnum:]]+)\.*?").unwrap();
        let undef_re = regex::Regex::new(r"(\w+) *?= *?([[:punct:][:alnum:]]+)\.*?").unwrap();
        let const_re = regex::Regex::new(r"const +(\w+) *?= *?([[:punct:][:alnum:]]+)\.*?").unwrap();

        let f = File::open(file_name)?;
        let f = BufReader::new(f);

        let mut variables: VariablesHeap = Box::new(Vec::new());

        for (linum, line) in f.lines().enumerate() {

            let text = line.unwrap().to_string();

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
            for cap in const_re.captures_iter(text.as_str()) {
                let var = Variable::new(&cap[1], VariableKind::Const, &cap[2], linum);
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

        }

        let _rep = Report::new(variables, vec![
            Box::new(NameValidity{}),
        ]);

        return Result::Ok(_rep.generate())
    }

}
