use mctext::{McText as RustMcText, NamedColor, Span as RustSpan, Style as RustStyle, TextColor};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct McText {
    inner: RustMcText,
}

#[derive(Serialize, Deserialize)]
pub struct Span {
    pub text: String,
    pub color: Option<Color>,
    pub style: Style,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Color {
    Named {
        name: String,
        code: char,
        rgb: [u8; 3],
    },
    Rgb {
        r: u8,
        g: u8,
        b: u8,
    },
}

#[derive(Serialize, Deserialize)]
pub struct Style {
    pub bold: bool,
    pub italic: bool,
    pub underlined: bool,
    pub strikethrough: bool,
    pub obfuscated: bool,
}

impl From<&RustStyle> for Style {
    fn from(s: &RustStyle) -> Self {
        Style {
            bold: s.bold,
            italic: s.italic,
            underlined: s.underlined,
            strikethrough: s.strikethrough,
            obfuscated: s.obfuscated,
        }
    }
}

impl From<TextColor> for Color {
    fn from(c: TextColor) -> Self {
        match c {
            TextColor::Named(named) => {
                let (r, g, b) = named.rgb();
                Color::Named {
                    name: named.name().to_string(),
                    code: named.code(),
                    rgb: [r, g, b],
                }
            }
            TextColor::Rgb { r, g, b } => Color::Rgb { r, g, b },
        }
    }
}

impl From<&RustSpan> for Span {
    fn from(s: &RustSpan) -> Self {
        Span {
            text: s.text.clone(),
            color: s.color.map(Color::from),
            style: Style::from(&s.style),
        }
    }
}

#[wasm_bindgen]
impl McText {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: RustMcText::new(),
        }
    }

    pub fn parse(text: &str) -> Self {
        Self {
            inner: RustMcText::parse(text),
        }
    }

    #[wasm_bindgen(js_name = parseJson)]
    pub fn parse_json(json: &str) -> Result<McText, JsError> {
        mctext::try_parse_json_component(json)
            .map(|inner| Self { inner })
            .map_err(|e| JsError::new(&e.to_string()))
    }

    #[wasm_bindgen(js_name = plainText)]
    pub fn plain_text(&self) -> String {
        self.inner.plain_text()
    }

    #[wasm_bindgen(js_name = toLegacy)]
    pub fn to_legacy(&self) -> String {
        self.inner.to_legacy()
    }

    #[wasm_bindgen(js_name = toJson)]
    pub fn to_json(&self) -> String {
        mctext::to_json(&self.inner)
    }

    pub fn spans(&self) -> JsValue {
        let spans: Vec<Span> = self.inner.spans().iter().map(Span::from).collect();
        serde_wasm_bindgen::to_value(&spans).unwrap_or(JsValue::NULL)
    }

    #[wasm_bindgen(js_name = charCount)]
    pub fn char_count(&self) -> usize {
        self.inner.char_count()
    }

    #[wasm_bindgen(js_name = isEmpty)]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl Default for McText {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen(js_name = stripCodes)]
pub fn strip_codes(text: &str) -> String {
    mctext::strip_codes(text)
}

#[wasm_bindgen(js_name = countVisibleChars)]
pub fn count_visible_chars(text: &str) -> usize {
    mctext::count_visible_chars(text)
}

#[wasm_bindgen(js_name = namedColors)]
pub fn named_colors() -> JsValue {
    let colors: Vec<_> = NamedColor::ALL
        .iter()
        .map(|c| {
            let (r, g, b) = c.rgb();
            serde_json::json!({
                "name": c.name(),
                "code": c.code().to_string(),
                "rgb": [r, g, b]
            })
        })
        .collect();
    serde_wasm_bindgen::to_value(&colors).unwrap_or(JsValue::NULL)
}

#[cfg(feature = "render")]
mod render {
    use super::*;
    use mctext::{
        FontFamily as RustFontFamily, FontSystem as RustFontSystem, FontVersion,
        LayoutOptions as RustLayoutOptions,
    };

    #[wasm_bindgen]
    #[derive(Clone, Copy)]
    pub enum FontFamily {
        Minecraft,
        Enchanting,
        Illager,
    }

    impl From<FontFamily> for RustFontFamily {
        fn from(f: FontFamily) -> Self {
            match f {
                FontFamily::Minecraft => RustFontFamily::Minecraft,
                FontFamily::Enchanting => RustFontFamily::Enchanting,
                FontFamily::Illager => RustFontFamily::Illager,
            }
        }
    }

    #[wasm_bindgen]
    pub struct FontSystem {
        inner: RustFontSystem,
    }

    #[wasm_bindgen]
    impl FontSystem {
        pub fn modern() -> Self {
            Self {
                inner: RustFontSystem::new(FontVersion::Modern),
            }
        }

        pub fn legacy() -> Self {
            Self {
                inner: RustFontSystem::new(FontVersion::Legacy),
            }
        }

        pub fn measure(&self, text: &str, size: f32) -> f32 {
            self.inner.measure_text(text, size)
        }

        #[wasm_bindgen(js_name = measureFamily)]
        pub fn measure_family(&self, text: &str, size: f32, family: FontFamily) -> f32 {
            self.inner.measure_text_family(text, size, family.into())
        }
    }

    #[wasm_bindgen]
    pub struct LayoutOptions {
        size: f32,
        max_width: Option<f32>,
        shadow: bool,
    }

    #[wasm_bindgen]
    impl LayoutOptions {
        #[wasm_bindgen(constructor)]
        pub fn new(size: f32) -> Self {
            Self {
                size,
                max_width: None,
                shadow: false,
            }
        }

        #[wasm_bindgen(js_name = withMaxWidth)]
        pub fn with_max_width(mut self, width: f32) -> Self {
            self.max_width = Some(width);
            self
        }

        #[wasm_bindgen(js_name = withShadow)]
        pub fn with_shadow(mut self, shadow: bool) -> Self {
            self.shadow = shadow;
            self
        }

        fn to_rust(&self) -> RustLayoutOptions {
            let mut opts = RustLayoutOptions::new(self.size);
            if let Some(w) = self.max_width {
                opts = opts.with_max_width(w);
            }
            opts = opts.with_shadow(self.shadow);
            opts
        }
    }

    #[wasm_bindgen]
    pub struct RenderResult {
        width: u32,
        height: u32,
        data: Vec<u8>,
    }

    #[wasm_bindgen]
    impl RenderResult {
        pub fn width(&self) -> u32 {
            self.width
        }

        pub fn height(&self) -> u32 {
            self.height
        }

        pub fn data(&self) -> Vec<u8> {
            self.data.clone()
        }
    }

    #[wasm_bindgen]
    pub fn render(
        font_system: &FontSystem,
        text: &str,
        width: u32,
        height: u32,
        options: &LayoutOptions,
    ) -> RenderResult {
        use mctext::{SoftwareRenderer, TextRenderContext};

        let ctx = TextRenderContext::new(&font_system.inner);
        let mut renderer =
            SoftwareRenderer::new(&font_system.inner, width as usize, height as usize);

        let _ = ctx.render_str(&mut renderer, text, 0.0, 0.0, &options.to_rust());

        RenderResult {
            width,
            height,
            data: renderer.buffer,
        }
    }

    #[wasm_bindgen(js_name = renderFamily)]
    pub fn render_family(
        font_system: &FontSystem,
        text: &str,
        width: u32,
        height: u32,
        size: f32,
        family: FontFamily,
    ) -> RenderResult {
        let rust_family: RustFontFamily = family.into();
        let mut buffer = vec![0u8; (width * height * 4) as usize];

        let font = font_system.inner.font_for_family(rust_family);
        let ascent = font
            .horizontal_line_metrics(size)
            .map(|m| m.ascent)
            .unwrap_or(size * 0.8);

        let mut x = 0.0f32;
        let y = ascent;

        for ch in text.chars() {
            if ch == ' ' {
                x += size * 0.4;
                continue;
            }
            if ch.is_control() {
                continue;
            }

            let (metrics, bitmap) = font.rasterize(ch, size);
            let gx = (x + metrics.xmin as f32) as i32;
            let gy = (y - metrics.height as f32 - metrics.ymin as f32) as i32;

            for row in 0..metrics.height {
                for col in 0..metrics.width {
                    let px = gx + col as i32;
                    let py = gy + row as i32;

                    if px < 0 || py < 0 || px >= width as i32 || py >= height as i32 {
                        continue;
                    }

                    let alpha = bitmap[row * metrics.width + col];
                    if alpha > 0 {
                        let idx = ((py as u32 * width + px as u32) * 4) as usize;
                        if idx + 3 < buffer.len() {
                            buffer[idx] = 255;
                            buffer[idx + 1] = 255;
                            buffer[idx + 2] = 255;
                            buffer[idx + 3] = alpha;
                        }
                    }
                }
            }

            x += metrics.advance_width;
        }

        RenderResult {
            width,
            height,
            data: buffer,
        }
    }
}

#[cfg(feature = "render")]
pub use render::*;
