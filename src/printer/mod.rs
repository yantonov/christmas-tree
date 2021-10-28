use termion::terminal_size;

pub trait LinePrinter {
    fn print(&self,
             lines: &[String]);
}

pub struct DummyLinePrinter {}

impl LinePrinter for DummyLinePrinter {
    fn print(&self,
             lines: &[String]) {
        for line in lines {
            println!("{}", line)
        }
    }
}

pub struct TerminalPrinter {}

impl LinePrinter for TerminalPrinter {
    fn print(&self,
             lines: &[String]) {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        let term_size = terminal_size().unwrap();
        let center = (term_size.0 - lines.get(0).unwrap().len() as u16) / 2;
        for (pos, line) in lines.iter().enumerate() {
            print!("{}", termion::cursor::Goto(center, 1 + pos as u16));
            println!("{}", line)
        }
    }
}

pub struct HtmlPrinter {}

impl LinePrinter for HtmlPrinter {
    fn print(&self, lines: &[String]) {
        println!("<table><tr><td>");
        println!("<pre style='background-color:black; padding: 20px'>");
        for line in lines {
            println!("{}<br/>", line)
        }
        println!("</pre>");
        println!("</td></tr></table>");
    }
}