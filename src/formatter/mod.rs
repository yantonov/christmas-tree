pub mod terminal;
pub mod dummy;
pub mod html;

use crate::line::StyledToken;

pub trait StyledTokenFormatter {
    fn format(&self, instance: &StyledToken) -> String;
}