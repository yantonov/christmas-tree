mod line;
mod formatter;
mod tree;
mod printer;

use crate::tree::ChristmasTree;
use crate::formatter::terminal::TerminalStyleTokenFormatter;
use crate::printer::{TerminalPrinter, LinePrinter};
use crate::formatter::format_grid;

fn main() {
    let printer = TerminalPrinter {};
    let tree = ChristmasTree::new(25);
    printer.print(&format_grid(&tree.render(),
                               &TerminalStyleTokenFormatter {}));
}