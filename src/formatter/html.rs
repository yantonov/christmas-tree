use crate::formatter::StyledTokenFormatter;
use crate::line::{StyledToken, Color};

pub struct HtmlStyledTokenFormatter {}

fn get_color_string(color: &Color) -> String {
    match color {
        Color::Red => "red".to_string(),
        Color::Green => "green".to_string(),
        Color::Blue => "blue".to_string(),
        Color::Yellow => "yellow".to_string(),
        Color::Cyan => "cyan".to_string(),
        Color::Magenta => "magenta".to_string(),
    }
}

impl StyledTokenFormatter for HtmlStyledTokenFormatter {
    fn format(&self, instance: &StyledToken) -> String {
        let style = instance.style();
        let color = style.get_color();
        match color {
            None => {
                instance.to_string()
            }
            Some(c) => {
                format!("<span style='color: {};'>{}</span>",
                        get_color_string(c),
                        instance.to_string())
            }
        }
    }
}
