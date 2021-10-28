use crate::line::{StyledToken, Color};
use crate::formatter::StyledTokenFormatter;

pub struct TerminalStyleTokenFormatter {}

fn get_color_string(color: &Color) -> String {
    match color {
        Color::Red => termion::color::Red.fg_str().to_string(),
        Color::Green => termion::color::Green.fg_str().to_string(),
        Color::Blue => termion::color::Blue.fg_str().to_string(),
        Color::Yellow => termion::color::Yellow.fg_str().to_string(),
        Color::Cyan => termion::color::Cyan.fg_str().to_string(),
        Color::Magenta => termion::color::Magenta.fg_str().to_string(),
    }
}

impl StyledTokenFormatter for TerminalStyleTokenFormatter {
    fn format(&self, instance: &StyledToken) -> String {
        let style = instance.style();
        let color = style.get_color();
        match color {
            None => {
                instance.to_string()
            }
            Some(c) => {
                format!("{}{}{}{}",
                        get_color_string(c),
                        instance.to_string(),
                        termion::color::Reset.bg_str(),
                        termion::color::Reset.fg_str())
            }
        }
    }
}