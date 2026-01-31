#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Style {
    pub bold: bool,
    pub italic: bool,
    pub underlined: bool,
    pub strikethrough: bool,
    pub obfuscated: bool,
}

impl Style {
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    pub fn underlined(mut self) -> Self {
        self.underlined = true;
        self
    }

    pub fn strikethrough(mut self) -> Self {
        self.strikethrough = true;
        self
    }

    pub fn obfuscated(mut self) -> Self {
        self.obfuscated = true;
        self
    }

    pub fn from_code(code: char) -> Option<Style> {
        match code.to_ascii_lowercase() {
            'l' => Some(Style::default().bold()),
            'o' => Some(Style::default().italic()),
            'n' => Some(Style::default().underlined()),
            'm' => Some(Style::default().strikethrough()),
            'k' => Some(Style::default().obfuscated()),
            _ => None,
        }
    }

    pub fn code(&self) -> Option<char> {
        if self.bold {
            Some('l')
        } else if self.italic {
            Some('o')
        } else if self.underlined {
            Some('n')
        } else if self.strikethrough {
            Some('m')
        } else if self.obfuscated {
            Some('k')
        } else {
            None
        }
    }

    pub fn is_empty(&self) -> bool {
        *self == Self::default()
    }

    pub fn merge(&self, other: &Style) -> Style {
        Style {
            bold: self.bold || other.bold,
            italic: self.italic || other.italic,
            underlined: self.underlined || other.underlined,
            strikethrough: self.strikethrough || other.strikethrough,
            obfuscated: self.obfuscated || other.obfuscated,
        }
    }
}

pub fn is_format_code(code: char) -> bool {
    matches!(code.to_ascii_lowercase(), 'l' | 'o' | 'n' | 'm' | 'k' | 'r')
}

pub fn is_reset_code(code: char) -> bool {
    code.eq_ignore_ascii_case(&'r')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_style() {
        let style = Style::default().bold().italic();
        assert!(style.bold && style.italic && !style.underlined);
        assert_eq!(Style::from_code('l'), Some(Style::default().bold()));
    }
}
