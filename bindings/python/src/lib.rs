use ::mctext::{McText as RustMcText, NamedColor, Span as RustSpan, Style as RustStyle, TextColor};
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
pub struct McText {
    inner: RustMcText,
}

#[pymethods]
impl McText {
    #[new]
    fn new() -> Self {
        Self {
            inner: RustMcText::new(),
        }
    }

    #[staticmethod]
    fn parse(text: &str) -> Self {
        Self {
            inner: RustMcText::parse(text),
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

    fn char_count(&self) -> usize {
        self.inner.char_count()
    }

    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    fn __repr__(&self) -> String {
        format!("McText('{}')", self.inner.plain_text())
    }

    fn __str__(&self) -> String {
        self.inner.plain_text()
    }

    fn __len__(&self) -> usize {
        self.inner.char_count()
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
        FontSystem as RustFontSystem, FontVariant, FontVersion, LayoutOptions as RustLayoutOptions,
        SoftwareRenderer, TextRenderContext,
    };

    #[pyclass]
    pub struct FontSystem {
        inner: RustFontSystem,
    }

    #[pymethods]
    impl FontSystem {
        #[staticmethod]
        #[cfg(feature = "modern-fonts")]
        fn modern() -> Self {
            Self {
                inner: RustFontSystem::new(FontVersion::Modern),
            }
        }

        #[staticmethod]
        #[cfg(feature = "legacy-fonts")]
        fn legacy() -> Self {
            Self {
                inner: RustFontSystem::new(FontVersion::Legacy),
            }
        }

        fn measure(&self, text: &str, size: f32) -> f32 {
            self.inner.measure_text(text, size)
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
    }

    #[pymethods]
    impl LayoutOptions {
        #[new]
        #[pyo3(signature = (size, max_width=None, shadow=false))]
        fn new(size: f32, max_width: Option<f32>, shadow: bool) -> Self {
            Self {
                size,
                max_width,
                shadow,
            }
        }
    }

    impl LayoutOptions {
        fn to_rust(&self) -> RustLayoutOptions {
            let mut opts = RustLayoutOptions::new(self.size);
            if let Some(w) = self.max_width {
                opts = opts.with_max_width(w);
            }
            opts = opts.with_shadow(self.shadow);
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
        text: &str,
        width: u32,
        height: u32,
        options: &LayoutOptions,
    ) -> RenderResult {
        let ctx = TextRenderContext::new(&font_system.inner);
        let mut renderer =
            SoftwareRenderer::new(&font_system.inner, width as usize, height as usize);

        // layout_at treats y as top of text area, not baseline
        // It internally adds ascent to position the baseline correctly
        let _ = ctx.render_str(&mut renderer, text, 0.0, 0.0, &options.to_rust());

        RenderResult {
            width,
            height,
            data: renderer.buffer,
        }
    }

    pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
        m.add_class::<FontSystem>()?;
        m.add_class::<LayoutOptions>()?;
        m.add_class::<RenderResult>()?;
        m.add_function(wrap_pyfunction!(render, m)?)?;
        Ok(())
    }
}

#[pymodule]
fn mctext(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<McText>()?;
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
