use crate::line::{Line, FormattedCharacter, formatted_str};
use termion::color;
use rand::Rng;

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

    pub fn render(&self) -> Vec<String> {
        let mut lines: Vec<String> = vec![];
        let line = Line::new(self.width);
        let mut rng = rand::thread_rng();

        lines.push(line.pad(&formatted_str("*", &color::Fg(color::Red))));
        for size in (3..self.width).step_by(2) {
            let mut chars: Vec<FormattedCharacter> = vec![];
            let ball_index = rng.gen::<u16>() % size;
            let email_index = rng.gen::<u16>() % size;
            let plus_index = rng.gen::<u16>() % size;
            for index in 0..size {
                chars.push(
                    if index == ball_index {
                        FormattedCharacter::styled_str(
                            "o",
                            &color::Fg(color::Yellow))
                    } else {
                        if index == email_index {
                            FormattedCharacter::styled_str(
                                "@",
                                &color::Fg(color::Cyan))
                        } else {
                            if index == plus_index {
                                FormattedCharacter::styled_str(
                                    "+",
                                    &color::Fg(color::Red))
                            } else {
                                FormattedCharacter::styled_str(
                                    "^",
                                    &color::Fg(color::Green))
                            }
                        }
                    });
            }
            lines.push(line.pad(&chars));
        }
        lines.push(line.fill(&FormattedCharacter::styled_str("#", &color::Fg(color::Blue))));
        lines.push(line.pad(&formatted_str("III", &color::Fg(color::Magenta))));
        lines.push(line.pad(&formatted_str("III", &color::Fg(color::Magenta))));
        lines.push(line.fill(&FormattedCharacter::styled_str("~", &color::Fg(color::Magenta))));
        lines.push(line.pad(&formatted_str(" MERRY CHRISTMAS", &color::Fg(color::Red))));
        lines.push(line.pad(&formatted_str("AND", &color::Fg(color::Red))));
        lines.push(line.pad(&formatted_str(" HAPPY HOLIDAYS!", &color::Fg(color::Red))));
        lines.push(line.fill(&FormattedCharacter::styled_str("~", &color::Fg(color::Red))));
        lines
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render() {
        let tree = ChristmasTree::new(25);
        let rendered = tree.render();
        assert_eq!(20, rendered.len())
    }
}

