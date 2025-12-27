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
    pub const NONE: Style = Style {
        bold: false,
        italic: false,
        underlined: false,
        strikethrough: false,
        obfuscated: false,
    };

    pub fn new() -> Self {
        Self::default()
    }

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
            'l' => Some(Style::NONE.bold()),
            'o' => Some(Style::NONE.italic()),
            'n' => Some(Style::NONE.underlined()),
            'm' => Some(Style::NONE.strikethrough()),
            'k' => Some(Style::NONE.obfuscated()),
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
        *self == Self::NONE
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
    fn test_style_builder() {
        let style = Style::new().bold().italic();
        assert!(style.bold);
        assert!(style.italic);
        assert!(!style.underlined);
    }

    #[test]
    fn test_from_code() {
        assert_eq!(Style::from_code('l'), Some(Style::NONE.bold()));
        assert_eq!(Style::from_code('O'), Some(Style::NONE.italic()));
        assert_eq!(Style::from_code('x'), None);
    }

    #[test]
    fn test_merge() {
        let a = Style::new().bold();
        let b = Style::new().italic();
        let merged = a.merge(&b);
        assert!(merged.bold);
        assert!(merged.italic);
    }
}
