pub mod dummy;
pub mod html;
pub mod terminal;

use crate::line::StyledToken;

pub trait StyledTokenFormatter {
    fn format(&self, instance: &StyledToken) -> String;
}
