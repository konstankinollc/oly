use std::{fs::File, fmt, io::{self, BufReader, prelude::*}};
use clap::{Arg, App};
use colored::*;

extern crate clap;
extern crate regex;

extern crate colored;

#[derive(Debug, PartialEq)]
enum VariableKind {
    Var,
    Let,
    Global,
}

#[derive(Debug, PartialEq)]
enum VariableScope {
    Unknown,
    // for now we do not take the scope into consideration
}

#[derive(Debug)]
struct VariableValue {
    value: String,
    kind: VariableValueKind,
}

#[derive(Debug)]
enum VariableValueKind {
    Compound,
    Dynamic,
}

impl Variable {

    fn new(name: &str, kind: VariableKind, value: &str, linum: usize) -> Variable {

        let dynamic_kind_re = regex::Regex::new(r"\(.*\)").unwrap();

        let var_value = VariableValue{value: value.to_owned(), kind: match dynamic_kind_re.is_match(value) {
            true => VariableValueKind::Dynamic,
            false => VariableValueKind::Compound,
        }};

        let var = Variable {
            name: name.to_owned(),
            kind: kind,
            value: var_value,
            scope: VariableScope::Unknown,
            linum: linum,
        };

        var
    }

}

impl PartialEq for Variable {
    fn eq(&self, other: &Variable) -> bool {
        self.name == other.name && self.kind == other.kind && self.scope == other.scope
    }
}

trait Recommender {
    fn suggest(&self, variable: &Variable) -> ReportLine;
}

struct NameValidity {}

#[derive(Debug, PartialEq)]
struct ReportLine {
    title: String,
    variable_name: String,
}

impl fmt::Display for ReportLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " {} {}", '\u{2937}', self.title)
    }
}

#[derive(Debug)]
struct Variable {
    name: String,
    kind: VariableKind,
    scope: VariableScope,
    value: VariableValue,
    linum: usize,
}

impl NameValidity {

    fn named_appropriately(&self, name: &str) -> bool {
        let appropriate_names = vec![
            "_",
            "self",
            "cors",
            "url",
            "ajax",
            "xhr",
            "id",
            "elem",
            "href",
            "data",
            "fn",
            "key",
            "obj",
            "tag",
            "body",
            "list",
            "dir",
            "attr",
            "len",
            "tag",
            "node",
            "rhs",
            "lhs",
            "win",
            "min",
            "max",
            "doc",
            "ret",
            "xml",
        ];
        appropriate_names.contains(&name)
    }

    fn starts_with_capical(&self, name: &str) -> bool {
        let all_capitals_re = regex::Regex::new(r"^[A-Z]").unwrap();
        all_capitals_re.is_match(name)
    }

    fn named_ijk(&self, name: &str) -> bool {
        for ch in "ijke".chars() {
            if name == ch.to_string() {
                return true
            }
        }
        false
    }

    fn includes_poor_chars(&self, name: &str) -> bool {
        for ch in "&$-_".chars() {
            if name.contains(ch) && name.chars().next().unwrap() != '_' {
                return true
            }
        }
        false
    }
}

impl Recommender for NameValidity {

    fn suggest(&self, variable: &Variable) -> ReportLine {

        const MAX_LENGTH: usize = 25;
        const MIN_LENGTH: usize = 5;

        match variable {

            Variable { name, .. } if self.includes_poor_chars(name) =>
                return ReportLine{
                    title: format!("Line {} Variable '{}' has terrible char in its name. Please consider renaming it.", variable.linum, variable.name),
                    variable_name: variable.name.to_string()
                },

            Variable { name, .. } if self.starts_with_capical(name) =>
                return ReportLine{
                    title: format!("Line {} Variable '{}' starts with Capital. Please come up with a better name", variable.linum, variable.name),
                    variable_name: variable.name.to_string()
                },

            Variable { name: y, .. } if ((y.len() > MAX_LENGTH || y.len() < MIN_LENGTH) && !self.named_ijk(y) && !self.named_appropriately(y)) =>
                return ReportLine{
                    title: format!("Line {} Variable '{}' seems odd. Please come up with a better name", variable.linum, variable.name),
                    variable_name: variable.name.to_string()
                },

            _ => return ReportLine{
                title: "".to_string(),
                variable_name: variable.name.to_string()
            },
        };
    }
}

struct Report {
    recommenders: Vec<Box<dyn Recommender>>,
    variables: Box<Vec<Variable>>,
}

impl Report {

    fn new(variables: Box<Vec<Variable>>, recommenders: Vec<Box<dyn Recommender>>) -> Report {
        Report {
            variables: variables,
            recommenders: recommenders,
        }
    }

    fn generate(&self) -> Box<Vec<ReportLine>> {
        let mut report: Box<Vec<ReportLine>> = Box::new(Vec::new());
        for rec in self.recommenders.iter() {
            for var in self.variables.iter() {
                let issue = rec.suggest(var);
                if !issue.title.is_empty() && !report.contains(&issue) {
                    report.push(issue);
                }
            }
        }
        report
    }

}

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
