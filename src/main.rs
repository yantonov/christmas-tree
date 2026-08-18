mod cli;
mod formatter;
mod line;
mod printer;
mod tree;

use crate::cli::{Command, Format};
use crate::formatter::StyledTokenFormatter;
use crate::formatter::dummy::DummyStyledTokenFormatter;
use crate::formatter::html::HtmlStyledTokenFormatter;
use crate::formatter::terminal::TerminalStyleTokenFormatter;
use crate::printer::{DummyLinePrinter, HtmlPrinter, LinePrinter, TerminalPrinter};
use crate::tree::ChristmasTree;

fn entry_point() -> Result<(), String> {
    let args = cli::arguments();
    match args.command() {
        Command::Show(show) => {
            let (printer, formatter): (&dyn LinePrinter, &dyn StyledTokenFormatter) =
                match show.format()? {
                    Format::Raw => (&DummyLinePrinter {}, &DummyStyledTokenFormatter {}),
                    Format::Term => (&TerminalPrinter {}, &TerminalStyleTokenFormatter {}),
                    Format::Html => (&HtmlPrinter {}, &HtmlStyledTokenFormatter {}),
                };
            let tree = ChristmasTree::new(show.width()?);
            printer.print(formatter, &tree.render());
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
