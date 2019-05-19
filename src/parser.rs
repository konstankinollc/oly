use std::{
    fs::File,
    io::{
        self,
        BufReader,
        prelude::*
    }
};

use crate::report::{
    Report
};
use crate::recommenders::{
    VariableKind,
    Variable,
    name_validity::NameValidity,
    ReportLine
};

extern crate regex;

pub struct Parser {}
pub struct Reader {}

type ReportLineHeap = Box<Vec<ReportLine>>;
type VariablesHeap = Box<Vec<Variable>>;

impl Reader {

    pub fn read_file(file_name: &str) -> io::Result<Box<Vec<(usize, String)>>> {

        let f = File::open(file_name)?;
        let f = BufReader::new(f);
        let mut result: Box<Vec<(usize, String)>> = Box::new(vec![]);

        for (linum, line) in f.lines().enumerate() {
            result.push((linum, line.unwrap().to_string()));
        }
        Ok(result)
    }
}

impl Parser {

    pub fn parse_loc(line_loc_pair: io::Result<Box<Vec<(usize, String)>>>) -> io::Result<(ReportLineHeap)> {

        let let_re = regex::Regex::new(r"let +(\w+) *?={1} *?([[:punct:][:alnum:]]+)\.*?").unwrap();
        let var_re = regex::Regex::new(r"var +(\w+) *?={1} *?([[:punct:][:alnum:]]+)\.*?").unwrap();
        let undef_re = regex::Regex::new(r"(\w+) *?={1} *?([[:punct:][:alnum:]]+)\.*?").unwrap();
        let const_re = regex::Regex::new(r"const +(\w+) *?={1} *?([[:punct:][:alnum:]]+)\.*?").unwrap();

        let mut variables: VariablesHeap = Box::new(Vec::new());

        for (linum, line) in line_loc_pair.unwrap().iter() {

            let text = line;

            if text.len() < 4 {
                continue
            }

            for cap in let_re.captures_iter(text.as_str()) {
                let var = Variable::new(&cap[1], VariableKind::Let, &cap[2], *linum);
                if !variables.contains(&var) {
                    variables.push(var);
                }
            }
            for cap in var_re.captures_iter(text.as_str()) {
                let var = Variable::new(&cap[1], VariableKind::Var, &cap[2], *linum);
                if !variables.contains(&var) {
                    variables.push(var);
                }
            }
            for cap in const_re.captures_iter(text.as_str()) {
                let var = Variable::new(&cap[1], VariableKind::Const, &cap[2], *linum);
                if !variables.contains(&var) {
                    variables.push(var);
                }
            }
            for cap in undef_re.captures_iter(text.as_str()) {
                let var = Variable::new(&cap[1], VariableKind::Global, &cap[2], *linum);
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

#[cfg(test)]
mod tests {

    use super::Parser;

    fn test_me(loc: &'a str) -> &'a str {
        let _loc =   Ok(Box::new(vec![(0 as usize, loc.to_string())]));
        let result = Parser::parse_loc(_loc).unwrap();

        match result.as_slice() {
            [res] => return res.variable_name.as_str(),
            _ => panic!("No match"),
        }
    }

    #[test]
    fn it_works() {
        let res = test_me("var foo");
        assert_eq!(res, "foo")
    }

}
