use mctext::{
    FontSystem, FontVersion, LayoutOptions, MCText, NamedColor, SoftwareRenderer, TextRenderContext,
};

fn main() {
    let fonts = FontSystem::new(FontVersion::Modern);

    let text = MCText::new()
        .span("hello ")
        .color(NamedColor::Red)
        .then("world!")
        .color(NamedColor::Gold)
        .build();

    let options = LayoutOptions::new(32.0).with_shadow(true);
    let (width, height) = (200, 60);

    let mut buffer = vec![0u8; width * height * 4];
    for pixel in buffer.chunks_exact_mut(4) {
        pixel[0] = 24;
        pixel[1] = 24;
        pixel[2] = 24;
        pixel[3] = 255;
    }

    let mut renderer = SoftwareRenderer::new(&fonts, &mut buffer, width, height);
    let _ = TextRenderContext::new(&fonts).render(&mut renderer, &text, 10.0, 14.0, &options);

    image::save_buffer(
        "rust_output.png",
        &buffer,
        width as u32,
        height as u32,
        image::ColorType::Rgba8,
    )
    .unwrap();
}
