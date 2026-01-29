#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum NamedColor {
    Black,
    DarkBlue,
    DarkGreen,
    DarkAqua,
    DarkRed,
    DarkPurple,
    Gold,
    Gray,
    DarkGray,
    Blue,
    Green,
    Aqua,
    Red,
    LightPurple,
    Yellow,
    #[default]
    White,
}

struct ColorData {
    color: NamedColor,
    code: char,
    name: &'static str,
    rgb: (u8, u8, u8),
}

const COLOR_TABLE: [ColorData; 16] = [
    ColorData {
        color: NamedColor::Black,
        code: '0',
        name: "black",
        rgb: (0, 0, 0),
    },
    ColorData {
        color: NamedColor::DarkBlue,
        code: '1',
        name: "dark_blue",
        rgb: (0, 0, 170),
    },
    ColorData {
        color: NamedColor::DarkGreen,
        code: '2',
        name: "dark_green",
        rgb: (0, 170, 0),
    },
    ColorData {
        color: NamedColor::DarkAqua,
        code: '3',
        name: "dark_aqua",
        rgb: (0, 170, 170),
    },
    ColorData {
        color: NamedColor::DarkRed,
        code: '4',
        name: "dark_red",
        rgb: (170, 0, 0),
    },
    ColorData {
        color: NamedColor::DarkPurple,
        code: '5',
        name: "dark_purple",
        rgb: (170, 0, 170),
    },
    ColorData {
        color: NamedColor::Gold,
        code: '6',
        name: "gold",
        rgb: (255, 170, 0),
    },
    ColorData {
        color: NamedColor::Gray,
        code: '7',
        name: "gray",
        rgb: (170, 170, 170),
    },
    ColorData {
        color: NamedColor::DarkGray,
        code: '8',
        name: "dark_gray",
        rgb: (85, 85, 85),
    },
    ColorData {
        color: NamedColor::Blue,
        code: '9',
        name: "blue",
        rgb: (85, 85, 255),
    },
    ColorData {
        color: NamedColor::Green,
        code: 'a',
        name: "green",
        rgb: (85, 255, 85),
    },
    ColorData {
        color: NamedColor::Aqua,
        code: 'b',
        name: "aqua",
        rgb: (85, 255, 255),
    },
    ColorData {
        color: NamedColor::Red,
        code: 'c',
        name: "red",
        rgb: (255, 85, 85),
    },
    ColorData {
        color: NamedColor::LightPurple,
        code: 'd',
        name: "light_purple",
        rgb: (255, 85, 255),
    },
    ColorData {
        color: NamedColor::Yellow,
        code: 'e',
        name: "yellow",
        rgb: (255, 255, 85),
    },
    ColorData {
        color: NamedColor::White,
        code: 'f',
        name: "white",
        rgb: (255, 255, 255),
    },
];

impl NamedColor {
    pub const ALL: [NamedColor; 16] = [
        NamedColor::Black,
        NamedColor::DarkBlue,
        NamedColor::DarkGreen,
        NamedColor::DarkAqua,
        NamedColor::DarkRed,
        NamedColor::DarkPurple,
        NamedColor::Gold,
        NamedColor::Gray,
        NamedColor::DarkGray,
        NamedColor::Blue,
        NamedColor::Green,
        NamedColor::Aqua,
        NamedColor::Red,
        NamedColor::LightPurple,
        NamedColor::Yellow,
        NamedColor::White,
    ];

    fn data(self) -> &'static ColorData {
        &COLOR_TABLE[self as usize]
    }

    pub fn rgb(self) -> (u8, u8, u8) {
        self.data().rgb
    }

    pub fn code(self) -> char {
        self.data().code
    }

    pub fn name(self) -> &'static str {
        self.data().name
    }

    pub fn from_code(code: char) -> Option<NamedColor> {
        let code = code.to_ascii_lowercase();
        COLOR_TABLE.iter().find(|d| d.code == code).map(|d| d.color)
    }

    pub fn from_name(name: &str) -> Option<NamedColor> {
        let name_lower = name.to_lowercase();
        let lookup = match name_lower.as_str() {
            "grey" => "gray",
            "dark_grey" => "dark_gray",
            _ => &name_lower,
        };
        COLOR_TABLE
            .iter()
            .find(|d| d.name == lookup)
            .map(|d| d.color)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextColor {
    Named(NamedColor),
    Rgb { r: u8, g: u8, b: u8 },
}

impl TextColor {
    pub fn rgb(self) -> (u8, u8, u8) {
        match self {
            TextColor::Named(named) => named.rgb(),
            TextColor::Rgb { r, g, b } => (r, g, b),
        }
    }

    pub fn shadow_rgb(self) -> (u8, u8, u8) {
        let (r, g, b) = self.rgb();
        shadow_color(r, g, b)
    }

    pub fn from_hex(hex: &str) -> Option<TextColor> {
        let hex = hex.strip_prefix('#').unwrap_or(hex);
        if hex.len() != 6 {
            return None;
        }
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        Some(TextColor::Rgb { r, g, b })
    }

    pub fn to_hex(self) -> String {
        let (r, g, b) = self.rgb();
        format!("#{:02X}{:02X}{:02X}", r, g, b)
    }

    pub fn parse(s: &str) -> Option<TextColor> {
        if s.starts_with('#') {
            TextColor::from_hex(s)
        } else {
            NamedColor::from_name(s).map(TextColor::Named)
        }
    }
}

impl From<NamedColor> for TextColor {
    fn from(named: NamedColor) -> Self {
        TextColor::Named(named)
    }
}

impl From<(u8, u8, u8)> for TextColor {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        TextColor::Rgb { r, g, b }
    }
}

impl Default for TextColor {
    fn default() -> Self {
        TextColor::Named(NamedColor::White)
    }
}

#[cfg(feature = "serde")]
impl Serialize for TextColor {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            TextColor::Named(named) => serializer.serialize_str(named.name()),
            TextColor::Rgb { r, g, b } => {
                serializer.serialize_str(&format!("#{:02x}{:02x}{:02x}", r, g, b))
            }
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for TextColor {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        TextColor::parse(&s).ok_or_else(|| serde::de::Error::custom("invalid color"))
    }
}

pub fn shadow_color(r: u8, g: u8, b: u8) -> (u8, u8, u8) {
    (r / 4, g / 4, b / 4)
}

pub const SHADOW_OFFSET: i32 = 1;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_named_color_roundtrip() {
        for color in NamedColor::ALL {
            assert_eq!(NamedColor::from_code(color.code()), Some(color));
            assert_eq!(NamedColor::from_name(color.name()), Some(color));
        }
    }

    #[test]
    fn test_text_color_hex() {
        assert_eq!(
            TextColor::from_hex("#FF5555"),
            Some(TextColor::Rgb {
                r: 255,
                g: 85,
                b: 85
            })
        );
        assert_eq!(TextColor::Named(NamedColor::Red).to_hex(), "#FF5555");
    }
}
