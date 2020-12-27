use crate::line::{Line, Style, StyledToken, Color};
use rand::Rng;
use crate::formatter::{StyledTokenFormatter, format_vec};

pub struct ChristmasTree {
    width: u16,
}

impl ChristmasTree {
    pub fn new(width: u16) -> ChristmasTree {
        if width % 2 == 0 {
            panic!("width should be odd")
        }
        ChristmasTree { width }
    }

    pub fn render(&self, formatter: &dyn StyledTokenFormatter) -> Vec<String> {
        let mut grid: Vec<Vec<StyledToken>> = vec![];
        let line = Line::new(self.width);
        let mut rng = rand::thread_rng();

        grid.push(line.pad(&vec![
            StyledToken::styled("*", Style::color(Color::Red))]));
        for size in (3..self.width).step_by(2) {
            let mut chars: Vec<StyledToken> = vec![];
            let ball_index = rng.gen::<u16>() % size;
            let email_index = rng.gen::<u16>() % size;
            let plus_index = rng.gen::<u16>() % size;
            for index in 0..size {
                chars.push(
                    if index == ball_index {
                        StyledToken::styled(
                            "o",
                            Style::color(Color::Yellow))
                    } else {
                        if index == email_index {
                            StyledToken::styled(
                                "@",
                                Style::color(Color::Cyan))
                        } else {
                            if index == plus_index {
                                StyledToken::styled(
                                    "+",
                                    Style::color(Color::Red))
                            } else {
                                StyledToken::styled(
                                    "^",
                                    Style::color(Color::Green))
                            }
                        }
                    });
            }
            grid.push(line.pad(&chars));
        }
        grid.push(line.fill(
            &StyledToken::styled("#", Style::color(Color::Blue))));
        grid.push(line.pad(
            &vec![StyledToken::styled("III", Style::color(Color::Magenta))]));
        grid.push(line.pad(
            &vec![StyledToken::styled("III", Style::color(Color::Magenta))]));
        grid.push(line.fill(
            &StyledToken::styled("#", Style::color(Color::Magenta))));
        grid.push(line.pad(
            &vec![StyledToken::styled(" MERRY CHRISTMAS", Style::color(Color::Red))]));
        grid.push(line.pad(
            &vec![StyledToken::styled("AND", Style::color(Color::Red))]));
        grid.push(line.pad(
            &vec![StyledToken::styled(" HAPPY HOLIDAYS!", Style::color(Color::Red))]));
        grid.push(line.fill(
            &StyledToken::styled("#", Style::color(Color::Red))));

        let mut lines: Vec<String> = vec![];
        for chars in grid.iter() {
            lines.push(format_vec(formatter, chars))
        }
        lines
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formatter::dummy::DummyStyledTokenFormatter;

    #[test]
    fn render() {
        let tree = ChristmasTree::new(25);
        let rendered = tree.render(&DummyStyledTokenFormatter {});
        assert_eq!(20, rendered.len())
    }
}

