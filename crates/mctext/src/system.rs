#[cfg(feature = "special-fonts")]
use crate::fonts::{ENCHANTING_REGULAR, ILLAGER_REGULAR};
use crate::fonts::{FontFamily, FontVariant, FontVersion};
use crate::style::Style;
use fontdue::{Font, FontSettings, Metrics};
#[cfg(feature = "special-fonts")]
use std::sync::OnceLock;

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

pub struct FontSystem {
    version: FontVersion,
    regular: Font,
    bold: Font,
    italic: Font,
    bold_italic: Font,
}

impl FontSystem {
    pub fn new(version: FontVersion) -> Self {
        let settings = FontSettings::default();
        Self {
            version,
            regular: Font::from_bytes(FontVariant::Regular.data_for_version(version), settings)
                .expect("Failed to load regular font"),
            bold: Font::from_bytes(FontVariant::Bold.data_for_version(version), settings)
                .expect("Failed to load bold font"),
            italic: Font::from_bytes(FontVariant::Italic.data_for_version(version), settings)
                .expect("Failed to load italic font"),
            bold_italic: Font::from_bytes(
                FontVariant::BoldItalic.data_for_version(version),
                settings,
            )
            .expect("Failed to load bold italic font"),
        }
    }

    #[cfg(feature = "modern-fonts")]
    pub fn modern() -> Self {
        Self::new(FontVersion::Modern)
    }

    #[cfg(feature = "legacy-fonts")]
    pub fn legacy() -> Self {
        Self::new(FontVersion::Legacy)
    }

    pub fn version(&self) -> FontVersion {
        self.version
    }

    pub fn font(&self, variant: FontVariant) -> &Font {
        match variant {
            FontVariant::Regular => &self.regular,
            FontVariant::Bold => &self.bold,
            FontVariant::Italic => &self.italic,
            FontVariant::BoldItalic => &self.bold_italic,
        }
    }

    pub fn font_for_family(&self, family: FontFamily) -> &Font {
        match family {
            FontFamily::Minecraft => &self.regular,
            #[cfg(feature = "special-fonts")]
            FontFamily::Enchanting => enchanting_font(),
            #[cfg(feature = "special-fonts")]
            FontFamily::Illager => illager_font(),
        }
    }

    pub fn font_for_style(&self, style: &Style) -> &Font {
        self.font(FontVariant::from_style(style.bold, style.italic))
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

    pub fn measure_char_family(&self, ch: char, size: f32, family: FontFamily) -> f32 {
        if ch == ' ' {
            size * 0.4
        } else {
            self.font_for_family(family).metrics(ch, size).advance_width
        }
    }

    pub fn measure_text_family(&self, text: &str, size: f32, family: FontFamily) -> f32 {
        let mut width = 0.0;
        let font = self.font_for_family(family);

        for ch in text.chars() {
            if ch.is_control() {
                continue;
            }
            if ch == ' ' {
                width += size * 0.4;
            } else {
                width += font.metrics(ch, size).advance_width;
            }
        }

        width
    }

    pub fn metrics(&self, ch: char, size: f32, variant: FontVariant) -> GlyphMetrics {
        self.font(variant).metrics(ch, size).into()
    }

    pub fn rasterize(&self, ch: char, size: f32, variant: FontVariant) -> (GlyphMetrics, Vec<u8>) {
        let (metrics, bitmap) = self.font(variant).rasterize(ch, size);
        (metrics.into(), bitmap)
    }

    pub fn has_glyph(&self, ch: char, variant: FontVariant) -> bool {
        self.font(variant).lookup_glyph_index(ch) != 0
    }

    pub fn ascent_ratio(&self, variant: FontVariant) -> f32 {
        let size = 16.0;
        self.font(variant)
            .horizontal_line_metrics(size)
            .map(|m| m.ascent / size)
            .unwrap_or(0.8)
    }

    pub fn measure_char(&self, ch: char, size: f32, variant: FontVariant) -> f32 {
        if ch == ' ' {
            size * 0.4
        } else {
            self.metrics(ch, size, variant).advance_width
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
    fn test_font_system_creation() {
        let system = FontSystem::modern();
        assert_eq!(system.version(), FontVersion::Modern);
    }

    #[test]
    #[cfg(all(feature = "modern-fonts", feature = "legacy-fonts"))]
    fn test_legacy_font_system() {
        let legacy = FontSystem::legacy();
        assert_eq!(legacy.version(), FontVersion::Legacy);
    }

    #[test]
    #[cfg(feature = "modern-fonts")]
    fn test_has_glyph() {
        let system = FontSystem::modern();
        assert!(system.has_glyph('A', FontVariant::Regular));
        assert!(system.has_glyph('0', FontVariant::Regular));
    }

    #[test]
    #[cfg(feature = "modern-fonts")]
    fn test_measure_text() {
        let system = FontSystem::modern();
        let width = system.measure_text("Hello", 16.0);
        assert!(width > 0.0);
    }

    #[test]
    #[cfg(feature = "modern-fonts")]
    fn test_measure_skips_color_codes() {
        let system = FontSystem::modern();
        let plain = system.measure_text("Hello", 16.0);
        let colored = system.measure_text("\u{00A7}6Hello", 16.0);
        assert!((plain - colored).abs() < 0.001);
    }

    #[test]
    #[cfg(feature = "modern-fonts")]
    fn test_ascent_ratio() {
        let system = FontSystem::modern();
        let ratio = system.ascent_ratio(FontVariant::Regular);
        assert!(ratio > 0.0 && ratio < 1.0);
    }
}
