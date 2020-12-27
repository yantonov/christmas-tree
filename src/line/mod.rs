#[derive(Clone)]
pub enum Color {
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Magenta,
}

#[derive(Clone)]
pub struct Style {
    color: Option<Color>
}

impl Style {
    pub fn default() -> Style {
        Style {
            color: None
        }
    }

    pub fn color(color: Color) -> Style {
        Style {
            color: Some(color)
        }
    }

    pub fn get_color(&self) -> &Option<Color> {
        &self.color
    }
}

#[derive(Clone)]
pub struct StyledToken {
    s: String,
    style: Style,
}

impl StyledToken {
    #[allow(dead_code)]
    pub fn from_str(s: &str) -> StyledToken {
        StyledToken {
            s: s.to_string(),
            style: Style::default(),
        }
    }

    pub fn styled(s: &str, style: Style) -> StyledToken {
        StyledToken {
            s: s.to_string(),
            style,
        }
    }

    pub fn len(&self) -> usize {
        self.s.len()
    }

    pub fn style(&self) -> &Style {
        &self.style
    }
}

impl ToString for StyledToken {
    fn to_string(&self) -> String {
        self.s.clone()
    }
}

pub struct Line {
    width: u16
}

impl Line {
    pub fn pad(&self,
               s: &Vec<StyledToken>) -> Vec<StyledToken> {
        self.pad_with(s, " ")
    }

    pub fn pad_with(&self,
                    s: &Vec<StyledToken>,
                    pad_with: &str) -> Vec<StyledToken> {
        let w = self.width as usize;
        let s_len: usize = s.iter().map(|x| x.len()).sum();

        if s_len > w {
            panic!("too long")
        }

        let mut result: Vec<StyledToken> = vec![];
        let pad_left_size = (w - s_len) / 2;
        for _ in 0..pad_left_size {
            result.push(StyledToken::from_str(pad_with));
        }
        for ch in s.iter() {
            result.push((*ch).clone());
        }
        for _ in 0..(w - s_len - pad_left_size) {
            result.push(StyledToken::from_str(pad_with));
        }
        result
    }

    pub fn fill(&self, ch: &StyledToken) -> Vec<StyledToken> {
        let mut line: Vec<StyledToken> = vec![];
        for _ in 0..self.width {
            line.push((*ch).clone());
        }
        line
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
        let width: usize = 9;
        let line = Line::new(width as u16);
        let chars: Vec<StyledToken> = line.pad(&vec![StyledToken::from_str("*")]);
        assert_eq!(width, chars.len());
        let center = width / 2;
        assert_eq!("*", chars.get(center).unwrap().s);
        for i in 0..width {
            if i != center {
                assert_eq!(" ", chars.get(i).unwrap().s);
            }
        }
    }

    #[test]
    fn pad_token() {
        let width: usize = 9;
        let line = Line::new(width as u16);
        let chars = line.pad(&vec![StyledToken::from_str("III")]);

        let actual_width: usize = 7;
        assert_eq!(7, chars.len());

        let center = actual_width / 2;
        assert_eq!("III", chars.get(center).unwrap().s);
        for i in 0..actual_width {
            if i != center {
                assert_eq!(" ", chars.get(i).unwrap().s);
            }
        }
    }

    #[test]
    fn fill() {
        let width: usize = 9;
        let line = Line::new(width as u16);
        let chars = line.fill(&StyledToken::from_str("#"));
        assert_eq!(width, chars.len());
        for i in 0..chars.len() {
            assert_eq!("#", chars.get(i).unwrap().s);
        }
    }
}