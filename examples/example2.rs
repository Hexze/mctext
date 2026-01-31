use mctext::{
    FontSystem, FontVersion, LayoutOptions, MCText, NamedColor, SoftwareRenderer, TextRenderContext,
};

fn main() {
    let fonts = FontSystem::new(FontVersion::Modern);

    let legacy = MCText::parse("§cr§6a§ei§an§bb§9o§dw §ftext");

    let bold = MCText::new()
        .span("bold ")
        .color(NamedColor::Red)
        .bold()
        .then("& ")
        .color(NamedColor::Gray)
        .then("italic")
        .color(NamedColor::Aqua)
        .italic()
        .build();

    let wrapped = MCText::new()
        .span("this text wraps across multiple lines")
        .color(NamedColor::Gold)
        .build();

    let options = LayoutOptions::new(24.0)
        .with_shadow(true)
        .with_max_width(300.0);
    let (width, height) = (300, 120);

    let mut buffer = vec![0u8; width * height * 4];
    for pixel in buffer.chunks_exact_mut(4) {
        pixel[0] = 24;
        pixel[1] = 24;
        pixel[2] = 24;
        pixel[3] = 255;
    }

    let mut renderer = SoftwareRenderer::new(&fonts, &mut buffer, width, height);
    let ctx = TextRenderContext::new(&fonts);
    let _ = ctx.render(&mut renderer, &legacy, 10.0, 6.0, &options);
    let _ = ctx.render(&mut renderer, &bold, 10.0, 36.0, &options);
    let _ = ctx.render(&mut renderer, &wrapped, 10.0, 66.0, &options);

    image::save_buffer(
        "rust_output2.png",
        &buffer,
        width as u32,
        height as u32,
        image::ColorType::Rgba8,
    )
    .unwrap();
}
