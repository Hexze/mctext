use std::ops::{Add, AddAssign};

use crate::color::{NamedColor, TextColor};
use crate::style::{Style, is_format_code, is_reset_code};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Span {
    pub text: String,
    pub color: Option<TextColor>,
    pub style: Style,
}

impl Span {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            color: None,
            style: Style::default(),
        }
    }

    pub fn with_color(mut self, color: impl Into<TextColor>) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MCText {
    spans: Vec<Span>,
}

pub struct SpanBuilder {
    mctext: MCText,
    text: String,
    color: Option<TextColor>,
    style: Style,
}

impl SpanBuilder {
    pub fn color(mut self, color: impl Into<TextColor>) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn bold(mut self) -> Self {
        self.style.bold = true;
        self
    }

    pub fn italic(mut self) -> Self {
        self.style.italic = true;
        self
    }

    pub fn underlined(mut self) -> Self {
        self.style.underlined = true;
        self
    }

    pub fn strikethrough(mut self) -> Self {
        self.style.strikethrough = true;
        self
    }

    pub fn obfuscated(mut self) -> Self {
        self.style.obfuscated = true;
        self
    }

    pub fn then(mut self, text: impl Into<String>) -> SpanBuilder {
        self.mctext.spans.push(Span {
            text: self.text,
            color: self.color,
            style: self.style,
        });
        SpanBuilder {
            mctext: self.mctext,
            text: text.into(),
            color: None,
            style: Style::default(),
        }
    }

    pub fn build(mut self) -> MCText {
        self.mctext.spans.push(Span {
            text: self.text,
            color: self.color,
            style: self.style,
        });
        self.mctext
    }
}

impl MCText {
    pub fn new() -> Self {
        Self { spans: Vec::new() }
    }

    pub fn parse(text: &str) -> Self {
        let mut spans = Vec::new();
        let mut current_text = String::new();
        let mut current_color: Option<TextColor> = None;
        let mut current_style = Style::default();
        let mut chars = text.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '\u{00A7}' {
                if let Some(&code) = chars.peek() {
                    if !current_text.is_empty() {
                        spans.push(Span {
                            text: std::mem::take(&mut current_text),
                            color: current_color,
                            style: current_style,
                        });
                    }

                    chars.next();

                    if is_reset_code(code) {
                        current_color = None;
                        current_style = Style::default();
                    } else if let Some(named) = NamedColor::from_code(code) {
                        current_color = Some(TextColor::Named(named));
                        current_style = Style::default();
                    } else if is_format_code(code) {
                        if let Some(style) = Style::from_code(code) {
                            current_style = current_style.merge(&style);
                        }
                    }
                } else {
                    current_text.push(ch);
                }
            } else {
                current_text.push(ch);
            }
        }

        if !current_text.is_empty() {
            spans.push(Span {
                text: current_text,
                color: current_color,
                style: current_style,
            });
        }

        Self { spans }
    }

    pub fn spans(&self) -> &[Span] {
        &self.spans
    }

    pub fn into_spans(self) -> Vec<Span> {
        self.spans
    }

    pub(crate) fn push(&mut self, span: Span) {
        self.spans.push(span);
    }

    pub fn span(self, text: impl Into<String>) -> SpanBuilder {
        SpanBuilder {
            mctext: self,
            text: text.into(),
            color: None,
            style: Style::default(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.spans.is_empty() || self.spans.iter().all(|s| s.is_empty())
    }

    pub fn append(&mut self, other: MCText) {
        self.spans.extend(other.spans);
    }

    pub fn concat(mut self, other: MCText) -> Self {
        self.spans.extend(other.spans);
        self
    }

    pub fn plain_text(&self) -> String {
        self.spans.iter().map(|s| s.text.as_str()).collect()
    }

    pub fn to_legacy(&self) -> String {
        let mut result = String::new();

        for span in &self.spans {
            if let Some(TextColor::Named(color)) = span.color {
                result.push('\u{00A7}');
                result.push(color.code());
            }
            if span.style.bold {
                result.push_str("\u{00A7}l");
            }
            if span.style.italic {
                result.push_str("\u{00A7}o");
            }
            if span.style.underlined {
                result.push_str("\u{00A7}n");
            }
            if span.style.strikethrough {
                result.push_str("\u{00A7}m");
            }
            if span.style.obfuscated {
                result.push_str("\u{00A7}k");
            }
            result.push_str(&span.text);
        }

        result
    }
}

pub fn strip_codes(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\u{00A7}' {
            chars.next();
        } else {
            result.push(ch);
        }
    }

    result
}

pub fn count_visible_chars(text: &str) -> usize {
    let mut count = 0;
    let mut chars = text.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\u{00A7}' {
            chars.next();
        } else {
            count += 1;
        }
    }

    count
}

impl<'a> IntoIterator for &'a MCText {
    type Item = &'a Span;
    type IntoIter = std::slice::Iter<'a, Span>;

    fn into_iter(self) -> Self::IntoIter {
        self.spans.iter()
    }
}

impl Add for MCText {
    type Output = MCText;

    fn add(self, other: MCText) -> MCText {
        self.concat(other)
    }
}

impl AddAssign for MCText {
    fn add_assign(&mut self, other: MCText) {
        self.append(other);
    }
}

impl IntoIterator for MCText {
    type Item = Span;
    type IntoIter = std::vec::IntoIter<Span>;

    fn into_iter(self) -> Self::IntoIter {
        self.spans.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let text = MCText::parse("§6Hello §bWorld");
        assert_eq!(text.spans().len(), 2);
        assert_eq!(text.plain_text(), "Hello World");
    }

    #[test]
    fn test_builder() {
        let text = MCText::new()
            .span("Hello ")
            .color(NamedColor::Red)
            .then("World")
            .color(NamedColor::Blue)
            .build();

        assert_eq!(text.spans().len(), 2);
        assert_eq!(text.plain_text(), "Hello World");
    }

    #[test]
    fn test_concat() {
        let a = MCText::new().span("Hello ").color(NamedColor::Red).build();
        let b = MCText::new().span("World").color(NamedColor::Blue).build();

        assert_eq!((a + b).plain_text(), "Hello World");
    }
}
