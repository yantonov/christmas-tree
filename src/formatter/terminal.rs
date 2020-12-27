use crate::line::{FormattedCharacter, Color};
use crate::formatter::Formatter;

pub struct TerminalFormatter {}

fn get_color_string(color: &Color) -> String {
    match color {
        Color::Red => format!("{}", termion::color::Red.fg_str()),
        Color::Green => format!("{}", termion::color::Green.fg_str()),
        Color::Blue => format!("{}", termion::color::Blue.fg_str()),
        Color::Yellow => format!("{}", termion::color::Yellow.fg_str()),
        Color::Cyan => format!("{}", termion::color::Blue.fg_str()),
        Color::Magenta => format!("{}", termion::color::Magenta.fg_str()),
    }
}

impl Formatter for TerminalFormatter {
    fn format(&self, instance: &FormattedCharacter) -> String {
        let style = instance.style();
        let color = style.get_color();
        match color {
            None => {
                format!("{}", instance.to_string())
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