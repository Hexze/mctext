use crate::fonts::FontVariant;
use crate::layout::{LayoutEngine, LayoutOptions, TextLayout};
use crate::system::FontSystem;
use crate::text::McText;

pub trait TextRenderer {
    type Error;

    fn render_glyph(
        &mut self,
        ch: char,
        x: f32,
        y: f32,
        size: f32,
        color: (u8, u8, u8, u8),
        variant: FontVariant,
    ) -> Result<(), Self::Error>;

    fn render_layout(&mut self, layout: &TextLayout) -> Result<(), Self::Error> {
        for glyph in &layout.glyphs {
            let (r, g, b) = if glyph.is_shadow {
                glyph.color.shadow_rgb()
            } else {
                glyph.color.rgb()
            };

            self.render_glyph(
                glyph.ch,
                glyph.x,
                glyph.y,
                glyph.size,
                (r, g, b, 255),
                glyph.variant,
            )?;
        }
        Ok(())
    }
}

pub struct TextRenderContext<'a> {
    font_system: &'a FontSystem,
    layout_engine: LayoutEngine<'a>,
}

impl<'a> TextRenderContext<'a> {
    pub fn new(font_system: &'a FontSystem) -> Self {
        Self {
            font_system,
            layout_engine: LayoutEngine::new(font_system),
        }
    }

    pub fn font_system(&self) -> &FontSystem {
        self.font_system
    }

    pub fn layout(&self, text: &McText, options: &LayoutOptions) -> TextLayout {
        self.layout_engine.layout(text, options)
    }

    pub fn layout_at(&self, text: &McText, x: f32, y: f32, options: &LayoutOptions) -> TextLayout {
        self.layout_engine.layout_at(text, x, y, options)
    }

    pub fn render<R: TextRenderer>(
        &self,
        renderer: &mut R,
        text: &McText,
        x: f32,
        y: f32,
        options: &LayoutOptions,
    ) -> Result<(), R::Error> {
        let layout = self.layout_at(text, x, y, options);
        renderer.render_layout(&layout)
    }

    pub fn render_str<R: TextRenderer>(
        &self,
        renderer: &mut R,
        text: &str,
        x: f32,
        y: f32,
        options: &LayoutOptions,
    ) -> Result<(), R::Error> {
        let parsed = McText::parse(text);
        self.render(renderer, &parsed, x, y, options)
    }
}

pub struct RasterizedGlyph {
    pub bitmap: Vec<u8>,
    pub width: usize,
    pub height: usize,
    pub offset_x: i32,
    pub offset_y: i32,
}

pub struct SoftwareRenderer<'a> {
    font_system: &'a FontSystem,
    pub buffer: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

impl<'a> SoftwareRenderer<'a> {
    pub fn new(font_system: &'a FontSystem, width: usize, height: usize) -> Self {
        Self {
            font_system,
            buffer: vec![0; width * height * 4],
            width,
            height,
        }
    }

    pub fn clear(&mut self, color: (u8, u8, u8, u8)) {
        for pixel in self.buffer.chunks_exact_mut(4) {
            pixel[0] = color.0;
            pixel[1] = color.1;
            pixel[2] = color.2;
            pixel[3] = color.3;
        }
    }

    fn blend_pixel(&mut self, x: usize, y: usize, color: (u8, u8, u8, u8), alpha: u8) {
        if x >= self.width || y >= self.height {
            return;
        }

        let idx = (y * self.width + x) * 4;
        if idx + 3 >= self.buffer.len() {
            return;
        }

        let src_alpha = (alpha as u32 * color.3 as u32) / 255;
        if src_alpha == 0 {
            return;
        }

        let dst_alpha = self.buffer[idx + 3] as u32;
        let out_alpha = src_alpha + dst_alpha * (255 - src_alpha) / 255;

        if out_alpha == 0 {
            return;
        }

        let blend = |src: u8, dst: u8| -> u8 {
            let src = src as u32;
            let dst = dst as u32;
            ((src * src_alpha + dst * dst_alpha * (255 - src_alpha) / 255) / out_alpha) as u8
        };

        self.buffer[idx] = blend(color.0, self.buffer[idx]);
        self.buffer[idx + 1] = blend(color.1, self.buffer[idx + 1]);
        self.buffer[idx + 2] = blend(color.2, self.buffer[idx + 2]);
        self.buffer[idx + 3] = out_alpha as u8;
    }
}

impl<'a> TextRenderer for SoftwareRenderer<'a> {
    type Error = ();

    fn render_glyph(
        &mut self,
        ch: char,
        x: f32,
        y: f32,
        size: f32,
        color: (u8, u8, u8, u8),
        variant: FontVariant,
    ) -> Result<(), Self::Error> {
        if ch == ' ' || ch.is_control() {
            return Ok(());
        }

        let (metrics, bitmap) = self.font_system.rasterize(ch, size, variant);

        let gx = (x + metrics.xmin as f32) as i32;
        let gy = (y - metrics.height as f32 - metrics.ymin as f32) as i32;

        for row in 0..metrics.height {
            for col in 0..metrics.width {
                let px = gx + col as i32;
                let py = gy + row as i32;

                if px < 0 || py < 0 {
                    continue;
                }

                let alpha = bitmap[row * metrics.width + col];
                if alpha > 0 {
                    self.blend_pixel(px as usize, py as usize, color, alpha);
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_software_renderer() {
        let system = FontSystem::modern();
        let mut renderer = SoftwareRenderer::new(&system, 100, 50);
        renderer.clear((0, 0, 0, 255));

        let ctx = TextRenderContext::new(&system);
        let options = LayoutOptions::new(16.0);

        ctx.render_str(&mut renderer, "Hi", 10.0, 10.0, &options)
            .unwrap();

        let has_content = renderer
            .buffer
            .chunks(4)
            .any(|p| p[3] > 0 && (p[0] > 0 || p[1] > 0 || p[2] > 0));
        assert!(has_content);
    }

    #[test]
    fn test_render_colored() {
        let system = FontSystem::modern();
        let mut renderer = SoftwareRenderer::new(&system, 100, 50);
        renderer.clear((0, 0, 0, 0));

        let ctx = TextRenderContext::new(&system);
        let options = LayoutOptions::new(16.0).with_shadow(false);

        ctx.render_str(&mut renderer, "\u{00A7}cR", 10.0, 10.0, &options)
            .unwrap();

        let red_pixels: Vec<_> = renderer
            .buffer
            .chunks(4)
            .filter(|p| p[3] > 0 && p[0] > p[1] && p[0] > p[2])
            .collect();

        assert!(!red_pixels.is_empty());
    }

    #[test]
    fn test_render_context() {
        let system = FontSystem::modern();
        let ctx = TextRenderContext::new(&system);

        let text = McText::parse("Test");
        let options = LayoutOptions::new(16.0);
        let layout = ctx.layout(&text, &options);

        assert!(!layout.glyphs.is_empty());
    }
}
