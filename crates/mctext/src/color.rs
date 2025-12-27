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

    pub fn rgb(self) -> (u8, u8, u8) {
        match self {
            NamedColor::Black => (0, 0, 0),
            NamedColor::DarkBlue => (0, 0, 170),
            NamedColor::DarkGreen => (0, 170, 0),
            NamedColor::DarkAqua => (0, 170, 170),
            NamedColor::DarkRed => (170, 0, 0),
            NamedColor::DarkPurple => (170, 0, 170),
            NamedColor::Gold => (255, 170, 0),
            NamedColor::Gray => (170, 170, 170),
            NamedColor::DarkGray => (85, 85, 85),
            NamedColor::Blue => (85, 85, 255),
            NamedColor::Green => (85, 255, 85),
            NamedColor::Aqua => (85, 255, 255),
            NamedColor::Red => (255, 85, 85),
            NamedColor::LightPurple => (255, 85, 255),
            NamedColor::Yellow => (255, 255, 85),
            NamedColor::White => (255, 255, 255),
        }
    }

    pub fn code(self) -> char {
        match self {
            NamedColor::Black => '0',
            NamedColor::DarkBlue => '1',
            NamedColor::DarkGreen => '2',
            NamedColor::DarkAqua => '3',
            NamedColor::DarkRed => '4',
            NamedColor::DarkPurple => '5',
            NamedColor::Gold => '6',
            NamedColor::Gray => '7',
            NamedColor::DarkGray => '8',
            NamedColor::Blue => '9',
            NamedColor::Green => 'a',
            NamedColor::Aqua => 'b',
            NamedColor::Red => 'c',
            NamedColor::LightPurple => 'd',
            NamedColor::Yellow => 'e',
            NamedColor::White => 'f',
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            NamedColor::Black => "black",
            NamedColor::DarkBlue => "dark_blue",
            NamedColor::DarkGreen => "dark_green",
            NamedColor::DarkAqua => "dark_aqua",
            NamedColor::DarkRed => "dark_red",
            NamedColor::DarkPurple => "dark_purple",
            NamedColor::Gold => "gold",
            NamedColor::Gray => "gray",
            NamedColor::DarkGray => "dark_gray",
            NamedColor::Blue => "blue",
            NamedColor::Green => "green",
            NamedColor::Aqua => "aqua",
            NamedColor::Red => "red",
            NamedColor::LightPurple => "light_purple",
            NamedColor::Yellow => "yellow",
            NamedColor::White => "white",
        }
    }

    pub fn from_code(code: char) -> Option<NamedColor> {
        match code.to_ascii_lowercase() {
            '0' => Some(NamedColor::Black),
            '1' => Some(NamedColor::DarkBlue),
            '2' => Some(NamedColor::DarkGreen),
            '3' => Some(NamedColor::DarkAqua),
            '4' => Some(NamedColor::DarkRed),
            '5' => Some(NamedColor::DarkPurple),
            '6' => Some(NamedColor::Gold),
            '7' => Some(NamedColor::Gray),
            '8' => Some(NamedColor::DarkGray),
            '9' => Some(NamedColor::Blue),
            'a' => Some(NamedColor::Green),
            'b' => Some(NamedColor::Aqua),
            'c' => Some(NamedColor::Red),
            'd' => Some(NamedColor::LightPurple),
            'e' => Some(NamedColor::Yellow),
            'f' => Some(NamedColor::White),
            _ => None,
        }
    }

    pub fn from_name(name: &str) -> Option<NamedColor> {
        match name.to_lowercase().as_str() {
            "black" => Some(NamedColor::Black),
            "dark_blue" => Some(NamedColor::DarkBlue),
            "dark_green" => Some(NamedColor::DarkGreen),
            "dark_aqua" => Some(NamedColor::DarkAqua),
            "dark_red" => Some(NamedColor::DarkRed),
            "dark_purple" => Some(NamedColor::DarkPurple),
            "gold" => Some(NamedColor::Gold),
            "gray" | "grey" => Some(NamedColor::Gray),
            "dark_gray" | "dark_grey" => Some(NamedColor::DarkGray),
            "blue" => Some(NamedColor::Blue),
            "green" => Some(NamedColor::Green),
            "aqua" => Some(NamedColor::Aqua),
            "red" => Some(NamedColor::Red),
            "light_purple" => Some(NamedColor::LightPurple),
            "yellow" => Some(NamedColor::Yellow),
            "white" => Some(NamedColor::White),
            _ => None,
        }
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
    fn test_named_color_roundtrip_code() {
        for color in NamedColor::ALL {
            let code = color.code();
            let parsed = NamedColor::from_code(code).unwrap();
            assert_eq!(color, parsed);
        }
    }

    #[test]
    fn test_named_color_roundtrip_name() {
        for color in NamedColor::ALL {
            let name = color.name();
            let parsed = NamedColor::from_name(name).unwrap();
            assert_eq!(color, parsed);
        }
    }

    #[test]
    fn test_named_case_insensitive() {
        assert_eq!(NamedColor::from_code('A'), Some(NamedColor::Green));
        assert_eq!(NamedColor::from_code('F'), Some(NamedColor::White));
        assert_eq!(NamedColor::from_name("GOLD"), Some(NamedColor::Gold));
        assert_eq!(
            NamedColor::from_name("Dark_Blue"),
            Some(NamedColor::DarkBlue)
        );
    }

    #[test]
    fn test_text_color_from_hex() {
        assert_eq!(
            TextColor::from_hex("#FF5555"),
            Some(TextColor::Rgb {
                r: 255,
                g: 85,
                b: 85
            })
        );
        assert_eq!(
            TextColor::from_hex("AA00FF"),
            Some(TextColor::Rgb {
                r: 170,
                g: 0,
                b: 255
            })
        );
        assert_eq!(TextColor::from_hex("invalid"), None);
    }

    #[test]
    fn test_text_color_parse() {
        assert_eq!(
            TextColor::parse("red"),
            Some(TextColor::Named(NamedColor::Red))
        );
        assert_eq!(
            TextColor::parse("#FF5555"),
            Some(TextColor::Rgb {
                r: 255,
                g: 85,
                b: 85
            })
        );
    }

    #[test]
    fn test_text_color_to_hex() {
        assert_eq!(TextColor::Named(NamedColor::Red).to_hex(), "#FF5555");
        assert_eq!(
            TextColor::Rgb {
                r: 170,
                g: 0,
                b: 255
            }
            .to_hex(),
            "#AA00FF"
        );
    }

    #[test]
    fn test_shadow_color() {
        let color = TextColor::Named(NamedColor::White);
        assert_eq!(color.shadow_rgb(), (63, 63, 63));
    }
}
