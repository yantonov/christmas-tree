pub mod terminal;
pub mod dummy;

use crate::line::StyledToken;

pub trait StyledTokenFormatter {
    fn format(&self, instance: &StyledToken) -> String;
}

pub fn format_vec(formatter: &dyn StyledTokenFormatter,
                  v: &Vec<StyledToken>) -> String {
    let mut result: Vec<String> = vec![];
    for item in v {
        result.push(formatter.format(item))
    }
    result.join("")
}