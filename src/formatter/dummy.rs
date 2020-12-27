use crate::formatter::StyledTokenFormatter;
use crate::line::StyledToken;

pub struct DummyStyledTokenFormatter {}

impl StyledTokenFormatter for DummyStyledTokenFormatter {
    fn format(&self, instance: &StyledToken) -> String {
        format!("{}", instance.to_string())
    }
}