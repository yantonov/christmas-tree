use core::fmt;
use termion::color;

pub struct FormattedCharacter {
    s: String,
    style: String,
}

impl FormattedCharacter {
    pub fn from_str(s: &str) -> FormattedCharacter {
        FormattedCharacter {
            style: "".to_string(),
            s: s.to_string(),
        }
    }

    pub fn styled_str(s: &str, style: &dyn fmt::Display) -> FormattedCharacter {
        FormattedCharacter {
            style: style.to_string(),
            s: s.to_string(),
        }
    }
}

impl ToString for FormattedCharacter {
    fn to_string(&self) -> String {
        let has_style = self.style.len() == 0;
        format!("{}{}{}{}",
                self.style,
                self.s,
                if has_style {
                    ""
                } else {
                    color::Reset.fg_str()
                },
                if has_style {
                    ""
                } else {
                    color::Reset.bg_str()
                })
    }
}


pub fn formatted_str(s: &str, style: &dyn fmt::Display) -> Vec<FormattedCharacter> {
    let mut result: Vec<FormattedCharacter> = vec![];
    for c in s.chars().into_iter() {
        result.push(FormattedCharacter::styled_str(&format!("{}", c), style));
    }
    result
}

pub struct Line {
    width: u16
}

impl Line {
    pub fn pad(&self,
               s: &Vec<FormattedCharacter>) -> String {
        self.pad_with(s, " ")
    }

    pub fn pad_with(&self,
                    s: &Vec<FormattedCharacter>,
                    pad_with: &str) -> String {
        let w = self.width as usize;
        let s_len = s.len();

        if s_len > w {
            panic!("too long")
        }

        let mut result: Vec<String> = vec![];
        let pad_left_size = (w - s_len) / 2;
        for _ in 0..pad_left_size {
            result.push(pad_with.to_string());
        }
        for ch in s.iter() {
            result.push(ch.to_string());
        }
        for _ in 0..(w - s_len - pad_left_size) {
            result.push(pad_with.to_string());
        }
        result.join("")
    }

    pub fn fill(&self, ch: &FormattedCharacter) -> String {
        let mut line: Vec<String> = vec![];
        for _ in 0..self.width {
            line.push(ch.to_string());
        }
        line.join("")
    }

    pub fn new(width: u16) -> Line {
        Line { width }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pad() {
        let line = Line::new(9);
        assert_eq!("    *    ", line.pad(&vec![FormattedCharacter::from_str("*")]))
    }

    #[test]
    fn fill() {
        let line = Line::new(9);
        assert_eq!("#########", line.fill(&FormattedCharacter::from_str("#")))
    }
}