mod cli;
mod line;
mod formatter;
mod tree;
mod printer;

use crate::tree::ChristmasTree;
use crate::formatter::terminal::TerminalStyleTokenFormatter;
use crate::printer::{TerminalPrinter, LinePrinter, DummyLinePrinter, HtmlPrinter};
use crate::formatter::{format_grid, StyledTokenFormatter};
use crate::cli::{Format, Command};
use crate::formatter::dummy::DummyStyledTokenFormatter;
use crate::formatter::html::HtmlStyledTokenFormatter;

fn entry_point() -> Result<(), String> {
    let args = cli::arguments();
    match args.command() {
        Command::Show(show) => {
            let (printer, formatter): (&dyn LinePrinter, &dyn StyledTokenFormatter) = match show.format()? {
                Format::Raw => {
                    (&DummyLinePrinter {}, &DummyStyledTokenFormatter {})
                }
                Format::Term => {
                    (&TerminalPrinter {}, &TerminalStyleTokenFormatter {})
                }
                Format::Html => {
                    (&HtmlPrinter {}, &HtmlStyledTokenFormatter {})
                }
            };
            let tree = ChristmasTree::new(25);
            printer.print(&format_grid(&tree.render(),
                                       formatter));
        }
    }
    Ok(())
}

fn main() {
    match entry_point() {
        Ok(_) => std::process::exit(0),
        Err(message) => {
            eprintln!("{}", message);
            std::process::exit(1);
        }
    }
}