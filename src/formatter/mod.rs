pub mod terminal;
pub mod dummy;
pub mod html;

use crate::line::StyledToken;

pub trait StyledTokenFormatter {
    fn format(&self, instance: &StyledToken) -> String;
}

fn format_vec(formatter: &dyn StyledTokenFormatter,
              v: &Vec<StyledToken>) -> String {
    let mut result: Vec<String> = vec![];
    for item in v {
        result.push(formatter.format(item))
    }
    result.join("")
}

pub fn format_grid(grid: &Vec<Vec<StyledToken>>,
                   formatter: &dyn StyledTokenFormatter) -> Vec<String> {
    let mut lines: Vec<String> = vec![];
    for chars in grid.iter() {
        lines.push(format_vec(formatter, chars))
    }
    lines
}