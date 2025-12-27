use crate::color::TextColor;
use crate::fonts::FontVariant;
use crate::system::FontSystem;
use crate::text::McText;

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
            line_spacing: 2.0,
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

    pub fn layout(&self, text: &McText, options: &LayoutOptions) -> TextLayout {
        self.layout_at(text, 0.0, 0.0, options)
    }

    pub fn layout_at(&self, text: &McText, x: f32, y: f32, options: &LayoutOptions) -> TextLayout {
        let default_color = TextColor::default();
        let mut glyphs = Vec::new();
        let mut lines: Vec<Vec<PositionedGlyph>> = vec![Vec::new()];
        let mut cursor_x = 0.0f32;
        let mut max_width = 0.0f32;

        let ascent = self.font_system.ascent_ratio(FontVariant::Regular) * options.size;
        let line_height = options.size + options.line_spacing;
        let shadow_offset = (options.size / 8.0).max(1.0);

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

        let total_height = lines.len() as f32 * line_height;
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

            current_y += line_height;
        }

        TextLayout {
            glyphs,
            width: max_width,
            height: total_height,
        }
    }

    pub fn measure(&self, text: &McText, size: f32) -> (f32, f32) {
        let options = LayoutOptions::new(size).with_shadow(false);
        let layout = self.layout(text, &options);
        (layout.width, layout.height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::NamedColor;
    use crate::text::Span;

    fn test_system() -> FontSystem {
        FontSystem::modern()
    }

    #[test]
    fn test_layout_simple() {
        let system = test_system();
        let engine = LayoutEngine::new(&system);
        let text = McText::parse("Hello");
        let options = LayoutOptions::new(16.0).with_shadow(false);
        let layout = engine.layout(&text, &options);

        assert_eq!(layout.glyphs.len(), 5);
        assert!(layout.width > 0.0);
    }

    #[test]
    fn test_layout_with_shadow() {
        let system = test_system();
        let engine = LayoutEngine::new(&system);
        let text = McText::parse("Hi");
        let options = LayoutOptions::new(16.0);
        let layout = engine.layout(&text, &options);

        assert_eq!(layout.glyphs.len(), 4);

        let shadows: Vec<_> = layout.glyphs.iter().filter(|g| g.is_shadow).collect();
        let foreground: Vec<_> = layout.glyphs.iter().filter(|g| !g.is_shadow).collect();

        assert_eq!(shadows.len(), 2);
        assert_eq!(foreground.len(), 2);
    }

    #[test]
    fn test_layout_colored() {
        let system = test_system();
        let engine = LayoutEngine::new(&system);
        let text = McText::parse("\u{00A7}6Gold");
        let options = LayoutOptions::new(16.0).with_shadow(false);
        let layout = engine.layout(&text, &options);

        assert_eq!(layout.glyphs.len(), 4);
        for glyph in &layout.glyphs {
            assert_eq!(glyph.color, TextColor::Named(NamedColor::Gold));
        }
    }

    #[test]
    fn test_layout_newline() {
        let system = test_system();
        let engine = LayoutEngine::new(&system);

        let mut text = McText::new();
        text.push(Span::new("A\nB"));

        let options = LayoutOptions::new(16.0).with_shadow(false);
        let layout = engine.layout(&text, &options);

        assert_eq!(layout.glyphs.len(), 2);

        let y_values: Vec<f32> = layout.glyphs.iter().map(|g| g.y).collect();
        assert!(y_values[1] > y_values[0]);
    }

    #[test]
    fn test_layout_alignment() {
        let system = test_system();
        let engine = LayoutEngine::new(&system);
        let text = McText::parse("Hi");

        let left = engine.layout(
            &text,
            &LayoutOptions::new(16.0)
                .with_shadow(false)
                .with_align(TextAlign::Left),
        );
        let right = engine.layout(
            &text,
            &LayoutOptions::new(16.0)
                .with_shadow(false)
                .with_align(TextAlign::Right),
        );

        let left_x = left.glyphs.first().map(|g| g.x).unwrap_or(0.0);
        let right_x = right.glyphs.first().map(|g| g.x).unwrap_or(0.0);

        assert!(left_x <= right_x);
    }

    #[test]
    fn test_measure() {
        let system = test_system();
        let engine = LayoutEngine::new(&system);
        let text = McText::parse("Test");
        let (width, height) = engine.measure(&text, 16.0);

        assert!(width > 0.0);
        assert!(height > 0.0);
    }
}
