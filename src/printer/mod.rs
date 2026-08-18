use crate::formatter::StyledTokenFormatter;
use crate::line::StyledToken;
use crossterm::cursor::MoveTo;
use crossterm::terminal::{Clear, ClearType, size};

pub trait LinePrinter {
    fn print(&self, formatter: &dyn StyledTokenFormatter, lines: &[Vec<StyledToken>]);
}

fn format_line(formatter: &dyn StyledTokenFormatter, v: &[StyledToken]) -> String {
    let mut result: Vec<String> = vec![];
    for item in v {
        result.push(formatter.format(item))
    }
    result.join("")
}

pub struct DummyLinePrinter {}

impl LinePrinter for DummyLinePrinter {
    fn print(&self, formatter: &dyn StyledTokenFormatter, lines: &[Vec<StyledToken>]) {
        for line in lines {
            let formatted = format_line(formatter, line);
            println!("{}", formatted)
        }
    }
}

pub struct TerminalPrinter {}

impl LinePrinter for TerminalPrinter {
    fn print(&self, formatter: &dyn StyledTokenFormatter, lines: &[Vec<StyledToken>]) {
        print!("{}{}", Clear(ClearType::All), MoveTo(0, 0));
        let term_size = size().unwrap();
        let center = (term_size.0 - lines.first().unwrap().len() as u16) / 2;
        for (pos, line) in lines.iter().enumerate() {
            let formatted = format_line(formatter, line);
            print!("{}", MoveTo(center, pos as u16));
            println!("{}", formatted)
        }
    }
}

pub struct HtmlPrinter {}

impl LinePrinter for HtmlPrinter {
    fn print(&self, formatter: &dyn StyledTokenFormatter, lines: &[Vec<StyledToken>]) {
        println!("<table><tr><td>");
        println!("<pre style='background-color:black; padding: 20px'>");
        for line in lines {
            let formatted = format_line(formatter, line);
            println!("{}<br/>", formatted)
        }
        println!("</pre>");
        println!("</td></tr></table>");
    }
}
