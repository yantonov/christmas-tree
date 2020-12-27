use crate::formatter::Formatter;
use crate::line::FormattedCharacter;

pub struct DummyFormatter {}

impl Formatter for DummyFormatter {
    fn format(&self, instance: &FormattedCharacter) -> String {
        format!("{}", instance.to_string())
    }
}