use crate::fonts::{FontFamily, FontVariant, FontVersion};
use fontdue::{Font, FontSettings, Metrics};
use std::sync::OnceLock;

#[cfg(feature = "special-fonts")]
use crate::fonts::{ENCHANTING_REGULAR, ILLAGER_REGULAR};

#[cfg(feature = "special-fonts")]
static ENCHANTING_FONT: OnceLock<Font> = OnceLock::new();
#[cfg(feature = "special-fonts")]
static ILLAGER_FONT: OnceLock<Font> = OnceLock::new();

#[cfg(feature = "special-fonts")]
fn enchanting_font() -> &'static Font {
    ENCHANTING_FONT.get_or_init(|| {
        Font::from_bytes(ENCHANTING_REGULAR, FontSettings::default())
            .expect("Failed to load enchanting font")
    })
}

#[cfg(feature = "special-fonts")]
fn illager_font() -> &'static Font {
    ILLAGER_FONT.get_or_init(|| {
        Font::from_bytes(ILLAGER_REGULAR, FontSettings::default())
            .expect("Failed to load illager font")
    })
}

const SPACE_WIDTH_RATIO: f32 = 0.4;
const DEFAULT_ASCENT_RATIO: f32 = 0.8;

pub struct GlyphMetrics {
    pub advance_width: f32,
    pub width: usize,
    pub height: usize,
    pub xmin: i32,
    pub ymin: i32,
}

impl From<Metrics> for GlyphMetrics {
    fn from(m: Metrics) -> Self {
        Self {
            advance_width: m.advance_width,
            width: m.width,
            height: m.height,
            xmin: m.xmin,
            ymin: m.ymin,
        }
    }
}

pub struct FontSystem {
    version: FontVersion,
    regular: OnceLock<Font>,
    bold: OnceLock<Font>,
    italic: OnceLock<Font>,
    bold_italic: OnceLock<Font>,
}

impl FontSystem {
    pub fn new(version: FontVersion) -> Self {
        Self {
            version,
            regular: OnceLock::new(),
            bold: OnceLock::new(),
            italic: OnceLock::new(),
            bold_italic: OnceLock::new(),
        }
    }

    fn load_font(&self, variant: FontVariant) -> Font {
        let settings = FontSettings::default();
        Font::from_bytes(variant.data_for_version(self.version), settings)
            .expect("Failed to load font")
    }

    #[cfg(feature = "modern-fonts")]
    pub fn modern() -> Self {
        Self::new(FontVersion::Modern)
    }

    #[cfg(feature = "legacy-fonts")]
    pub fn legacy() -> Self {
        Self::new(FontVersion::Legacy)
    }

    fn font(&self, variant: FontVariant) -> &Font {
        match variant {
            FontVariant::Regular => self.regular.get_or_init(|| self.load_font(variant)),
            FontVariant::Bold => self.bold.get_or_init(|| self.load_font(variant)),
            FontVariant::Italic => self.italic.get_or_init(|| self.load_font(variant)),
            FontVariant::BoldItalic => self.bold_italic.get_or_init(|| self.load_font(variant)),
        }
    }

    fn metrics(&self, ch: char, size: f32, variant: FontVariant) -> GlyphMetrics {
        self.font(variant).metrics(ch, size).into()
    }

    pub fn font_for_family(&self, family: FontFamily) -> &Font {
        match family {
            FontFamily::Minecraft => self.font(FontVariant::Regular),
            #[cfg(feature = "special-fonts")]
            FontFamily::Enchanting => enchanting_font(),
            #[cfg(feature = "special-fonts")]
            FontFamily::Illager => illager_font(),
        }
    }

    pub fn rasterize(&self, ch: char, size: f32, variant: FontVariant) -> (GlyphMetrics, Vec<u8>) {
        let (metrics, bitmap) = self.font(variant).rasterize(ch, size);
        (metrics.into(), bitmap)
    }

    pub fn rasterize_family(
        &self,
        ch: char,
        size: f32,
        family: FontFamily,
    ) -> (GlyphMetrics, Vec<u8>) {
        let (metrics, bitmap) = self.font_for_family(family).rasterize(ch, size);
        (metrics.into(), bitmap)
    }

    pub fn ascent_ratio(&self, variant: FontVariant) -> f32 {
        let size = 16.0;
        self.font(variant)
            .horizontal_line_metrics(size)
            .map(|m| m.ascent / size)
            .unwrap_or(DEFAULT_ASCENT_RATIO)
    }

    pub fn measure_char(&self, ch: char, size: f32, variant: FontVariant) -> f32 {
        if ch == ' ' {
            size * SPACE_WIDTH_RATIO
        } else {
            self.metrics(ch, size, variant).advance_width
        }
    }

    pub fn measure_char_family(&self, ch: char, size: f32, family: FontFamily) -> f32 {
        if ch == ' ' {
            size * SPACE_WIDTH_RATIO
        } else {
            self.font_for_family(family).metrics(ch, size).advance_width
        }
    }

    pub fn measure_text(&self, text: &str, size: f32) -> f32 {
        self.measure_text_styled(text, size, FontVariant::Regular)
    }

    pub fn measure_text_styled(&self, text: &str, size: f32, variant: FontVariant) -> f32 {
        let mut width = 0.0;
        let mut chars = text.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '\u{00A7}' {
                chars.next();
                continue;
            }
            if ch.is_control() {
                continue;
            }
            width += self.measure_char(ch, size, variant);
        }

        width
    }

    pub fn measure_text_family(&self, text: &str, size: f32, family: FontFamily) -> f32 {
        let mut width = 0.0;
        for ch in text.chars() {
            if ch.is_control() {
                continue;
            }
            width += self.measure_char_family(ch, size, family);
        }
        width
    }
}

#[cfg(feature = "modern-fonts")]
impl Default for FontSystem {
    fn default() -> Self {
        Self::modern()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "modern-fonts")]
    fn test_font_system() {
        let system = FontSystem::modern();
        assert!(system.measure_text("Hello", 16.0) > 0.0);
    }

    #[test]
    #[cfg(feature = "modern-fonts")]
    fn test_measure_skips_color_codes() {
        let system = FontSystem::modern();
        let plain = system.measure_text("Hello", 16.0);
        let colored = system.measure_text("§6Hello", 16.0);
        assert!((plain - colored).abs() < 0.001);
    }
}
