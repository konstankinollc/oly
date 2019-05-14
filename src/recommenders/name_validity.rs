use crate::recommenders::{Recommender, Variable, ReportLine};

pub struct NameValidity {}

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
