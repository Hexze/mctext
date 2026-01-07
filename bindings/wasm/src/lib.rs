use mctext::{
    MCText as RustMCText, NamedColor, Span as RustSpan, SpanBuilder as RustSpanBuilder,
    Style as RustStyle, TextColor,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct MCText {
    inner: RustMCText,
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
impl MCText {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: RustMCText::new(),
        }
    }

    pub fn parse(text: &str) -> Self {
        Self {
            inner: RustMCText::parse(text),
        }
    }

    #[wasm_bindgen(js_name = parseJson)]
    pub fn parse_json(json: &str) -> Result<MCText, JsError> {
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

    pub fn span(self, text: &str) -> SpanBuilder {
        SpanBuilder {
            inner: Some(self.inner.span(text)),
        }
    }
}

impl Default for MCText {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
pub struct SpanBuilder {
    inner: Option<RustSpanBuilder>,
}

#[wasm_bindgen]
impl SpanBuilder {
    pub fn color(mut self, color: &str) -> Self {
        if let Some(inner) = self.inner.take() {
            if let Some(parsed) = TextColor::parse(color) {
                self.inner = Some(inner.color(parsed));
            } else {
                self.inner = Some(inner);
            }
        }
        self
    }

    pub fn bold(mut self) -> Self {
        if let Some(inner) = self.inner.take() {
            self.inner = Some(inner.bold());
        }
        self
    }

    pub fn italic(mut self) -> Self {
        if let Some(inner) = self.inner.take() {
            self.inner = Some(inner.italic());
        }
        self
    }

    pub fn underlined(mut self) -> Self {
        if let Some(inner) = self.inner.take() {
            self.inner = Some(inner.underlined());
        }
        self
    }

    pub fn strikethrough(mut self) -> Self {
        if let Some(inner) = self.inner.take() {
            self.inner = Some(inner.strikethrough());
        }
        self
    }

    pub fn obfuscated(mut self) -> Self {
        if let Some(inner) = self.inner.take() {
            self.inner = Some(inner.obfuscated());
        }
        self
    }

    pub fn then(mut self, text: &str) -> Self {
        if let Some(inner) = self.inner.take() {
            self.inner = Some(inner.then(text));
        }
        self
    }

    pub fn build(mut self) -> MCText {
        MCText {
            inner: self.inner.take().map(|b| b.build()).unwrap_or_default(),
        }
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
    use mctext::{FontSystem as RustFontSystem, FontVersion, LayoutOptions as RustLayoutOptions};

    #[wasm_bindgen]
    pub struct FontSystem {
        inner: RustFontSystem,
    }

    #[wasm_bindgen]
    impl FontSystem {
        #[cfg(feature = "modern-fonts")]
        pub fn modern() -> Self {
            Self {
                inner: RustFontSystem::new(FontVersion::Modern),
            }
        }

        #[cfg(feature = "legacy-fonts")]
        pub fn legacy() -> Self {
            Self {
                inner: RustFontSystem::new(FontVersion::Legacy),
            }
        }

        pub fn measure(&self, text: &str, size: f32) -> f32 {
            self.inner.measure_text(text, size)
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
        text: &MCText,
        width: u32,
        height: u32,
        options: &LayoutOptions,
    ) -> RenderResult {
        use mctext::{SoftwareRenderer, TextRenderContext};

        let ctx = TextRenderContext::new(&font_system.inner);
        let mut renderer =
            SoftwareRenderer::new(&font_system.inner, width as usize, height as usize);

        let _ = ctx.render(&mut renderer, &text.inner, 0.0, 0.0, &options.to_rust());

        RenderResult {
            width,
            height,
            data: renderer.buffer,
        }
    }
}

#[cfg(feature = "render")]
pub use render::*;
