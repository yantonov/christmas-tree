mod line;
mod tree;

use crate::tree::ChristmasTree;
use termion::terminal_size;

fn main() {
    print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
    let tree_width: u16 = 25;
    let tree = ChristmasTree::new(25);
    let rendered = tree.render();
    let term_size = terminal_size().unwrap();
    let center = (term_size.0 - tree_width) / 2;
    for (pos, line) in rendered.iter().enumerate() {
        print!("{}", termion::cursor::Goto(center, 1 + pos as u16));
        println!("{}", line);
    }
}