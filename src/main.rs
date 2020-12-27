mod line;
mod formatter;
mod tree;
mod printer;

use crate::tree::ChristmasTree;
use crate::formatter::terminal::TerminalStyleTokenFormatter;
use crate::printer::{TerminalPrinter, LinePrinter, DummyLinePrinter};

fn main() {
    let printer = DummyLinePrinter {};
    let tree = ChristmasTree::new(25);
    printer.print(&tree.render(&TerminalStyleTokenFormatter {}));
}