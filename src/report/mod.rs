use std::fmt;
use crate::recommenders::{Recommender, Variable, ReportLine};

impl fmt::Display for ReportLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " {} {}", '\u{2937}', self.title)
    }
}

pub struct Report {
    recommenders: Vec<Box<dyn Recommender>>,
    variables: Box<Vec<Variable>>,
}

impl Report {

    pub fn new(variables: Box<Vec<Variable>>, recommenders: Vec<Box<dyn Recommender>>) -> Report {
        Report {
            variables: variables,
            recommenders: recommenders,
        }
    }

    pub fn generate(&self) -> Box<Vec<ReportLine>> {
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
