use mctext::{
    FontSystem, FontVersion, LayoutOptions, MCText, NamedColor, SoftwareRenderer, TextRenderContext,
};

struct TextRenderer {
    fonts: FontSystem,
}

impl TextRenderer {
    fn new() -> Self {
        Self {
            fonts: FontSystem::new(FontVersion::Modern),
        }
    }

    fn render(&self, text: &MCText, size: f32) -> (Vec<u8>, usize, usize) {
        let width = self.fonts.measure_text(&text.plain_text(), size) as usize + 8;
        let height = (size * 2.0) as usize;
        let options = LayoutOptions::new(size).with_shadow(true);
        let mut renderer = SoftwareRenderer::new(&self.fonts, width, height);

        let _ = TextRenderContext::new(&self.fonts).render(&mut renderer, text, 0.0, 0.0, &options);

        (renderer.buffer, width, height)
    }
}

fn composite(
    canvas: &mut [u8],
    canvas_width: usize,
    text: &[u8],
    text_width: usize,
    text_height: usize,
    x: usize,
    y: usize,
) {
    for ty in 0..text_height {
        for tx in 0..text_width {
            let src = (ty * text_width + tx) * 4;
            let dst = ((y + ty) * canvas_width + (x + tx)) * 4;
            let alpha = text[src + 3] as f32 / 255.0;
            for c in 0..3 {
                canvas[dst + c] =
                    (text[src + c] as f32 * alpha + canvas[dst + c] as f32 * (1.0 - alpha)) as u8;
            }
            canvas[dst + 3] = 255;
        }
    }
}

fn main() {
    let renderer = TextRenderer::new();

    let mut canvas = vec![0u8; 400 * 60 * 4];
    for i in (0..canvas.len()).step_by(4) {
        canvas[i + 3] = 255;
    }

    let text = MCText::new()
        .add("Minecraft Text!")
        .color(NamedColor::Red)
        .build();
    let (data, w, h) = renderer.render(&text, 16.0);
    composite(&mut canvas, 400, &data, w, h, 10, 20);

    image::save_buffer("rust_output.png", &canvas, 400, 60, image::ColorType::Rgba8).unwrap();
}
