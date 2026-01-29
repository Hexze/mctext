use crate::color::TextColor;
use crate::fonts::FontVariant;
use crate::system::FontSystem;
use crate::text::MCText;

const SHADOW_OFFSET_RATIO: f32 = 1.0 / 12.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextAlign {
    #[default]
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone)]
pub struct LayoutOptions {
    pub size: f32,
    pub max_width: Option<f32>,
    pub align: TextAlign,
    pub shadow: bool,
    pub line_spacing: f32,
}

impl Default for LayoutOptions {
    fn default() -> Self {
        Self {
            size: 16.0,
            max_width: None,
            align: TextAlign::Left,
            shadow: true,
            line_spacing: -1.0,
        }
    }
}

impl LayoutOptions {
    pub fn new(size: f32) -> Self {
        Self {
            size,
            ..Default::default()
        }
    }

    pub fn with_max_width(mut self, width: f32) -> Self {
        self.max_width = Some(width);
        self
    }

    pub fn with_align(mut self, align: TextAlign) -> Self {
        self.align = align;
        self
    }

    pub fn with_shadow(mut self, shadow: bool) -> Self {
        self.shadow = shadow;
        self
    }

    pub fn with_line_spacing(mut self, spacing: f32) -> Self {
        self.line_spacing = spacing;
        self
    }
}

#[derive(Debug, Clone)]
pub struct PositionedGlyph {
    pub ch: char,
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub color: TextColor,
    pub variant: FontVariant,
    pub is_shadow: bool,
}

#[derive(Debug, Clone)]
pub struct TextLayout {
    pub glyphs: Vec<PositionedGlyph>,
    pub width: f32,
    pub height: f32,
}

impl TextLayout {
    pub fn new() -> Self {
        Self {
            glyphs: Vec::new(),
            width: 0.0,
            height: 0.0,
        }
    }
}

impl Default for TextLayout {
    fn default() -> Self {
        Self::new()
    }
}

pub struct LayoutEngine<'a> {
    font_system: &'a FontSystem,
}

impl<'a> LayoutEngine<'a> {
    pub fn new(font_system: &'a FontSystem) -> Self {
        Self { font_system }
    }

    pub fn layout(&self, text: &MCText, options: &LayoutOptions) -> TextLayout {
        self.layout_at(text, 0.0, 0.0, options)
    }

    pub fn layout_at(&self, text: &MCText, x: f32, y: f32, options: &LayoutOptions) -> TextLayout {
        let default_color = TextColor::default();
        let mut glyphs = Vec::new();
        let mut lines: Vec<Vec<PositionedGlyph>> = vec![Vec::new()];
        let mut cursor_x = 0.0f32;
        let mut max_width = 0.0f32;

        let ascent = self.font_system.ascent_ratio(FontVariant::Regular) * options.size;
        let shadow_offset = options.size * SHADOW_OFFSET_RATIO;

        for span in text.spans() {
            let color = span.color.unwrap_or(default_color);
            let variant = FontVariant::from_style(span.style.bold, span.style.italic);

            for ch in span.text.chars() {
                if ch == '\n' {
                    max_width = max_width.max(cursor_x);
                    cursor_x = 0.0;
                    lines.push(Vec::new());
                    continue;
                }

                if ch.is_control() {
                    continue;
                }

                let advance = self.font_system.measure_char(ch, options.size, variant);

                if let Some(max_w) = options.max_width {
                    if cursor_x + advance > max_w && cursor_x > 0.0 {
                        max_width = max_width.max(cursor_x);
                        cursor_x = 0.0;
                        lines.push(Vec::new());
                    }
                }

                let glyph = PositionedGlyph {
                    ch,
                    x: cursor_x,
                    y: 0.0,
                    size: options.size,
                    color,
                    variant,
                    is_shadow: false,
                };

                if let Some(line) = lines.last_mut() {
                    line.push(glyph);
                }

                cursor_x += advance;
            }
        }

        max_width = max_width.max(cursor_x);

        let line_count = lines.len() as f32;
        let gap_count = (lines.len().saturating_sub(1)) as f32;
        let total_height = line_count * options.size + gap_count * options.line_spacing;
        let mut current_y = y + ascent;

        for line in &lines {
            let line_width: f32 = line
                .iter()
                .map(|g| self.font_system.measure_char(g.ch, options.size, g.variant))
                .sum();

            let x_offset = match options.align {
                TextAlign::Left => x,
                TextAlign::Center => x + (max_width - line_width) / 2.0,
                TextAlign::Right => x + max_width - line_width,
            };

            for glyph in line {
                let gx = x_offset + glyph.x;
                let gy = current_y;

                if options.shadow {
                    glyphs.push(PositionedGlyph {
                        ch: glyph.ch,
                        x: gx + shadow_offset,
                        y: gy + shadow_offset,
                        size: glyph.size,
                        color: glyph.color,
                        variant: glyph.variant,
                        is_shadow: true,
                    });
                }

                glyphs.push(PositionedGlyph {
                    ch: glyph.ch,
                    x: gx,
                    y: gy,
                    size: glyph.size,
                    color: glyph.color,
                    variant: glyph.variant,
                    is_shadow: false,
                });
            }

            current_y += options.size + options.line_spacing;
        }

        TextLayout {
            glyphs,
            width: max_width,
            height: total_height,
        }
    }

    pub fn measure(&self, text: &MCText, size: f32) -> (f32, f32) {
        let options = LayoutOptions::new(size).with_shadow(false);
        let layout = self.layout(text, &options);
        (layout.width, layout.height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn test_system() -> FontSystem {
        FontSystem::modern()
    }

    #[test]
    fn test_layout() {
        let system = test_system();
        let engine = LayoutEngine::new(&system);
        let text = MCText::parse("§6Hello");
        let layout = engine.layout(&text, &LayoutOptions::new(16.0));

        assert_eq!(layout.glyphs.len(), 10); // 5 chars + 5 shadows
        assert!(layout.width > 0.0);
    }

    #[test]
    fn test_measure() {
        let system = test_system();
        let engine = LayoutEngine::new(&system);
        let (width, height) = engine.measure(&MCText::parse("Test"), 16.0);

        assert!(width > 0.0 && height > 0.0);
    }
}
