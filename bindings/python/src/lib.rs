use ::mctext::{
    MCText as RustMCText, NamedColor, Span as RustSpan, SpanBuilder as RustSpanBuilder,
    Style as RustStyle, TextColor,
};
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
pub struct Style {
    #[pyo3(get)]
    bold: bool,
    #[pyo3(get)]
    italic: bool,
    #[pyo3(get)]
    underlined: bool,
    #[pyo3(get)]
    strikethrough: bool,
    #[pyo3(get)]
    obfuscated: bool,
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

#[pymethods]
impl Style {
    fn __repr__(&self) -> String {
        format!(
            "Style(bold={}, italic={}, underlined={}, strikethrough={}, obfuscated={})",
            self.bold, self.italic, self.underlined, self.strikethrough, self.obfuscated
        )
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Color {
    inner: TextColor,
}

#[pymethods]
impl Color {
    #[getter]
    fn r(&self) -> u8 {
        self.inner.rgb().0
    }

    #[getter]
    fn g(&self) -> u8 {
        self.inner.rgb().1
    }

    #[getter]
    fn b(&self) -> u8 {
        self.inner.rgb().2
    }

    #[getter]
    fn rgb(&self) -> (u8, u8, u8) {
        self.inner.rgb()
    }

    #[getter]
    fn name(&self) -> Option<String> {
        match self.inner {
            TextColor::Named(n) => Some(n.name().to_string()),
            TextColor::Rgb { .. } => None,
        }
    }

    #[getter]
    fn code(&self) -> Option<char> {
        match self.inner {
            TextColor::Named(n) => Some(n.code()),
            TextColor::Rgb { .. } => None,
        }
    }

    #[getter]
    fn is_named(&self) -> bool {
        matches!(self.inner, TextColor::Named(_))
    }

    fn to_hex(&self) -> String {
        self.inner.to_hex()
    }

    fn __repr__(&self) -> String {
        match self.inner {
            TextColor::Named(n) => format!("Color(name='{}')", n.name()),
            TextColor::Rgb { r, g, b } => format!("Color(r={}, g={}, b={})", r, g, b),
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Span {
    #[pyo3(get)]
    text: String,
    color: Option<Color>,
    style: Style,
}

#[pymethods]
impl Span {
    #[getter]
    fn color(&self) -> Option<Color> {
        self.color.clone()
    }

    #[getter]
    fn style(&self) -> Style {
        self.style.clone()
    }

    fn __repr__(&self) -> String {
        format!(
            "Span(text='{}', color={:?}, style={:?})",
            self.text,
            self.color.as_ref().map(|c| c.__repr__()),
            self.style.__repr__()
        )
    }
}

impl From<&RustSpan> for Span {
    fn from(s: &RustSpan) -> Self {
        Span {
            text: s.text.clone(),
            color: s.color.map(|c| Color { inner: c }),
            style: Style::from(&s.style),
        }
    }
}

#[pyclass]
pub struct MCText {
    inner: RustMCText,
}

#[pymethods]
impl MCText {
    #[new]
    fn new() -> Self {
        Self {
            inner: RustMCText::new(),
        }
    }

    #[staticmethod]
    fn parse(text: &str) -> Self {
        Self {
            inner: RustMCText::parse(text),
        }
    }

    #[staticmethod]
    fn parse_json(json: &str) -> PyResult<Self> {
        ::mctext::try_parse_json_component(json)
            .map(|inner| Self { inner })
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    fn plain_text(&self) -> String {
        self.inner.plain_text()
    }

    fn to_legacy(&self) -> String {
        self.inner.to_legacy()
    }

    fn to_json(&self) -> String {
        ::mctext::to_json(&self.inner)
    }

    fn spans(&self) -> Vec<Span> {
        self.inner.spans().iter().map(Span::from).collect()
    }

    fn __repr__(&self) -> String {
        format!("MCText('{}')", self.inner.plain_text())
    }

    fn __str__(&self) -> String {
        self.inner.plain_text()
    }

    fn __len__(&self) -> usize {
        self.inner.plain_text().chars().count()
    }

    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    fn concat(&self, other: &MCText) -> MCText {
        MCText {
            inner: self.inner.clone().concat(other.inner.clone()),
        }
    }

    fn span(slf: PyRef<'_, Self>, text: &str) -> SpanBuilder {
        SpanBuilder {
            inner: Some(slf.inner.clone().span(text)),
        }
    }
}

#[pyclass]
pub struct SpanBuilder {
    inner: Option<RustSpanBuilder>,
}

#[pymethods]
impl SpanBuilder {
    fn color(&mut self, color: &str) -> SpanBuilder {
        let inner = self.inner.take();
        SpanBuilder {
            inner: inner.map(|b| {
                if let Some(parsed) = TextColor::parse(color) {
                    b.color(parsed)
                } else {
                    b
                }
            }),
        }
    }

    fn bold(&mut self) -> SpanBuilder {
        SpanBuilder {
            inner: self.inner.take().map(|b| b.bold()),
        }
    }

    fn italic(&mut self) -> SpanBuilder {
        SpanBuilder {
            inner: self.inner.take().map(|b| b.italic()),
        }
    }

    fn underlined(&mut self) -> SpanBuilder {
        SpanBuilder {
            inner: self.inner.take().map(|b| b.underlined()),
        }
    }

    fn strikethrough(&mut self) -> SpanBuilder {
        SpanBuilder {
            inner: self.inner.take().map(|b| b.strikethrough()),
        }
    }

    fn obfuscated(&mut self) -> SpanBuilder {
        SpanBuilder {
            inner: self.inner.take().map(|b| b.obfuscated()),
        }
    }

    #[pyo3(name = "then")]
    fn then_span(&mut self, text: &str) -> SpanBuilder {
        SpanBuilder {
            inner: self.inner.take().map(|b| b.then(text)),
        }
    }

    fn build(&mut self) -> MCText {
        MCText {
            inner: self.inner.take().map(|b| b.build()).unwrap_or_default(),
        }
    }
}

#[pyfunction]
fn strip_codes(text: &str) -> String {
    ::mctext::strip_codes(text)
}

#[pyfunction]
fn count_visible_chars(text: &str) -> usize {
    ::mctext::count_visible_chars(text)
}

#[pyfunction]
fn named_colors() -> Vec<(String, char, (u8, u8, u8))> {
    NamedColor::ALL
        .iter()
        .map(|c| (c.name().to_string(), c.code(), c.rgb()))
        .collect()
}

#[cfg(feature = "render")]
mod rendering {
    use super::*;
    use ::mctext::{
        FontFamily as RustFontFamily, FontSystem as RustFontSystem, FontVariant, FontVersion,
        LayoutOptions as RustLayoutOptions, SoftwareRenderer, TextRenderContext,
    };

    #[pyclass(eq, eq_int)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum FontFamily {
        Minecraft,
        Enchanting,
        Illager,
    }

    impl From<FontFamily> for RustFontFamily {
        fn from(f: FontFamily) -> Self {
            match f {
                FontFamily::Minecraft => RustFontFamily::Minecraft,
                #[cfg(feature = "special-fonts")]
                FontFamily::Enchanting => RustFontFamily::Enchanting,
                #[cfg(feature = "special-fonts")]
                FontFamily::Illager => RustFontFamily::Illager,
                #[cfg(not(feature = "special-fonts"))]
                _ => RustFontFamily::Minecraft,
            }
        }
    }

    #[pyclass]
    pub struct FontSystem {
        inner: RustFontSystem,
    }

    #[pymethods]
    impl FontSystem {
        #[staticmethod]
        fn modern() -> Self {
            Self {
                inner: RustFontSystem::new(FontVersion::Modern),
            }
        }

        #[staticmethod]
        fn legacy() -> Self {
            Self {
                inner: RustFontSystem::new(FontVersion::Legacy),
            }
        }

        fn measure(&self, text: &str, size: f32) -> f32 {
            self.inner.measure_text(text, size)
        }

        fn measure_family(&self, text: &str, size: f32, family: FontFamily) -> f32 {
            self.inner.measure_text_family(text, size, family.into())
        }

        fn ascent_ratio(&self) -> f32 {
            self.inner.ascent_ratio(FontVariant::Regular)
        }
    }

    #[pyclass]
    #[derive(Clone)]
    pub struct LayoutOptions {
        size: f32,
        max_width: Option<f32>,
        shadow: bool,
        align: String,
        line_spacing: f32,
    }

    #[pymethods]
    impl LayoutOptions {
        #[new]
        fn new(size: f32) -> Self {
            Self {
                size,
                max_width: None,
                shadow: false,
                align: "left".to_string(),
                line_spacing: -1.0,
            }
        }

        fn with_max_width(&self, width: f32) -> Self {
            let mut opts = self.clone();
            opts.max_width = Some(width);
            opts
        }

        fn with_shadow(&self, shadow: bool) -> Self {
            let mut opts = self.clone();
            opts.shadow = shadow;
            opts
        }

        fn with_align(&self, align: &str) -> Self {
            let mut opts = self.clone();
            opts.align = align.to_string();
            opts
        }

        fn with_line_spacing(&self, spacing: f32) -> Self {
            let mut opts = self.clone();
            opts.line_spacing = spacing;
            opts
        }
    }

    impl LayoutOptions {
        fn to_rust(&self) -> RustLayoutOptions {
            use ::mctext::TextAlign;
            let mut opts = RustLayoutOptions::new(self.size);
            if let Some(w) = self.max_width {
                opts = opts.with_max_width(w);
            }
            opts = opts.with_shadow(self.shadow);
            opts = opts.with_line_spacing(self.line_spacing);
            opts = opts.with_align(match self.align.as_str() {
                "center" => TextAlign::Center,
                "right" => TextAlign::Right,
                _ => TextAlign::Left,
            });
            opts
        }
    }

    #[pyclass]
    pub struct RenderResult {
        #[pyo3(get)]
        width: u32,
        #[pyo3(get)]
        height: u32,
        data: Vec<u8>,
    }

    #[pymethods]
    impl RenderResult {
        fn data(&self) -> &[u8] {
            &self.data
        }

        fn to_bytes(&self) -> Vec<u8> {
            self.data.clone()
        }
    }

    #[pyfunction]
    pub fn render(
        font_system: &FontSystem,
        text: &MCText,
        width: u32,
        height: u32,
        options: &LayoutOptions,
    ) -> RenderResult {
        let (w, h) = (width as usize, height as usize);
        let mut buffer = vec![0u8; w * h * 4];

        {
            let mut renderer = SoftwareRenderer::new(&font_system.inner, &mut buffer, w, h);
            let ctx = TextRenderContext::new(&font_system.inner);
            let _ = ctx.render(&mut renderer, &text.inner, 0.0, 0.0, &options.to_rust());
        }

        RenderResult {
            width,
            height,
            data: buffer,
        }
    }

    #[pyfunction]
    pub fn render_family(
        font_system: &FontSystem,
        text: &str,
        width: u32,
        height: u32,
        size: f32,
        family: FontFamily,
    ) -> RenderResult {
        let (w, h) = (width as usize, height as usize);
        let mut buffer = vec![0u8; w * h * 4];
        let rust_family: RustFontFamily = family.into();
        let font = font_system.inner.font_for_family(rust_family);
        let ascent = font
            .horizontal_line_metrics(size)
            .map(|m| m.ascent)
            .unwrap_or(size * 0.8);

        let mut x = 0.0f32;
        for ch in text.chars() {
            if ch.is_control() {
                continue;
            }

            let (metrics, bitmap) = font.rasterize(ch, size);
            let gx = (x + metrics.xmin as f32) as i32;
            let gy = (ascent - metrics.ymin as f32 - metrics.height as f32) as i32;

            for row in 0..metrics.height {
                for col in 0..metrics.width {
                    let px = gx + col as i32;
                    let py = gy + row as i32;
                    if px >= 0 && px < w as i32 && py >= 0 && py < h as i32 {
                        let src = bitmap[row * metrics.width + col];
                        if src > 0 {
                            let idx = ((py as usize) * w + (px as usize)) * 4;
                            buffer[idx] = 255;
                            buffer[idx + 1] = 255;
                            buffer[idx + 2] = 255;
                            buffer[idx + 3] = src;
                        }
                    }
                }
            }

            x += if ch == ' ' {
                size * 0.4
            } else {
                metrics.advance_width
            };
        }

        RenderResult {
            width,
            height,
            data: buffer,
        }
    }

    pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
        m.add_class::<FontFamily>()?;
        m.add_class::<FontSystem>()?;
        m.add_class::<LayoutOptions>()?;
        m.add_class::<RenderResult>()?;
        m.add_function(wrap_pyfunction!(render, m)?)?;
        m.add_function(wrap_pyfunction!(render_family, m)?)?;
        Ok(())
    }
}

#[pymodule]
fn mctext(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<MCText>()?;
    m.add_class::<SpanBuilder>()?;
    m.add_class::<Span>()?;
    m.add_class::<Color>()?;
    m.add_class::<Style>()?;
    m.add_function(wrap_pyfunction!(strip_codes, m)?)?;
    m.add_function(wrap_pyfunction!(count_visible_chars, m)?)?;
    m.add_function(wrap_pyfunction!(named_colors, m)?)?;

    #[cfg(feature = "render")]
    rendering::register(m)?;

    Ok(())
}
