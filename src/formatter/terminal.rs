use crate::formatter::StyledTokenFormatter;
use crate::line::{Color, StyledToken};
use crossterm::style::{Color as TerminalColor, ResetColor, SetForegroundColor};

pub struct TerminalStyleTokenFormatter {}

fn get_color_string(color: &Color) -> String {
    let terminal_color = match color {
        Color::Red => TerminalColor::Red,
        Color::Green => TerminalColor::Green,
        Color::Blue => TerminalColor::Blue,
        Color::Yellow => TerminalColor::Yellow,
        Color::Cyan => TerminalColor::Cyan,
        Color::Magenta => TerminalColor::Magenta,
    };
    SetForegroundColor(terminal_color).to_string()
}

impl StyledTokenFormatter for TerminalStyleTokenFormatter {
    fn format(&self, instance: &StyledToken) -> String {
        let style = instance.style();
        let color = style.get_color();
        match color {
            None => instance.to_string(),
            Some(c) => {
                format!("{}{}{}", get_color_string(c), instance, ResetColor)
            }
        }
    }
}
