pub mod name_validity;

#[derive(Debug)]
pub struct Variable {
    name: String,
    kind: VariableKind,
    scope: VariableScope,
    value: VariableValue,
    linum: usize,
}

#[derive(Debug, PartialEq)]
pub enum VariableKind {
    Var,
    Let,
    Global,
    Const,
}

#[derive(Debug, PartialEq)]
pub enum VariableScope {
    Unknown,
    // for now we do not take the scope into consideration
}

#[derive(Debug)]
pub struct VariableValue {
    value: String,
    kind: VariableValueKind,
}

#[derive(Debug)]
pub enum VariableValueKind {
    Compound,
    Dynamic,
}

impl Variable {

    pub fn new(name: &str, kind: VariableKind, value: &str, linum: usize) -> Variable {

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

pub trait Recommender {
    fn suggest(&self, variable: &Variable) -> ReportLine;
}

#[derive(Debug, PartialEq)]
pub struct ReportLine {
    pub title: String,
    pub variable_name: String,
}

impl PartialEq for Variable {
    fn eq(&self, other: &Variable) -> bool {
        self.name == other.name && self.scope == other.scope
    }
}
