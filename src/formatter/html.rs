use crate::formatter::StyledTokenFormatter;
use crate::line::{StyledToken, Color};

pub struct HtmlStyledTokenFormatter {}

fn get_color_string(color: &Color) -> String {
    match color {
        Color::Red => format!("{}", "red"),
        Color::Green => format!("{}", "green"),
        Color::Blue => format!("{}", "blue"),
        Color::Yellow => format!("{}", "yellow"),
        Color::Cyan => format!("{}", "cyan"),
        Color::Magenta => format!("{}", "magenta"),
    }
}

impl StyledTokenFormatter for HtmlStyledTokenFormatter {
    fn format(&self, instance: &StyledToken) -> String {
        let style = instance.style();
        let color = style.get_color();
        match color {
            None => {
                format!("{}", instance.to_string())
            }
            Some(c) => {
                format!("<span style='color: {};'>{}</span>",
                        get_color_string(c),
                        instance.to_string())
            }
        }
    }
}
