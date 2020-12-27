pub mod terminal;
pub mod dummy;

use crate::line::FormattedCharacter;

pub trait Formatter {
    fn format(&self, instance: &FormattedCharacter) -> String;
}

pub fn format_vec(formatter: &dyn Formatter,
                  v: &Vec<FormattedCharacter>) -> String {
    let mut result: Vec<String> = vec![];
    for item in v {
        result.push(formatter.format(item))
    }
    result.join("")
}