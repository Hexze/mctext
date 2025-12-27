use crate::color::TextColor;
use crate::style::Style;
use crate::text::{McText, Span};
use serde_json::Value;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    InvalidJson(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidJson(msg) => write!(f, "invalid JSON: {}", msg),
        }
    }
}

impl std::error::Error for ParseError {}

pub fn try_parse_json_component(json: &str) -> Result<McText, ParseError> {
    let value =
        serde_json::from_str::<Value>(json).map_err(|e| ParseError::InvalidJson(e.to_string()))?;
    Ok(parse_value(&value))
}

pub fn parse_json_component(json: &str) -> McText {
    try_parse_json_component(json).unwrap_or_default()
}

pub fn parse_value(value: &Value) -> McText {
    let mut text = McText::new();
    extract_spans(value, None, Style::default(), &mut text);
    text
}

fn extract_spans(
    value: &Value,
    parent_color: Option<TextColor>,
    parent_style: Style,
    text: &mut McText,
) {
    match value {
        Value::String(s) => {
            if !s.is_empty() {
                let parsed = McText::parse(s);
                if parsed.is_empty() {
                    text.push(Span {
                        text: s.clone(),
                        color: parent_color,
                        style: parent_style,
                    });
                } else {
                    for mut span in parsed.into_spans() {
                        if span.color.is_none() {
                            span.color = parent_color;
                        }
                        if span.style.is_empty() {
                            span.style = parent_style;
                        }
                        text.push(span);
                    }
                }
            }
        }
        Value::Object(obj) => {
            let color = obj
                .get("color")
                .and_then(|v| v.as_str())
                .and_then(TextColor::parse)
                .or(parent_color);

            let style = Style {
                bold: obj
                    .get("bold")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(parent_style.bold),
                italic: obj
                    .get("italic")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(parent_style.italic),
                underlined: obj
                    .get("underlined")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(parent_style.underlined),
                strikethrough: obj
                    .get("strikethrough")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(parent_style.strikethrough),
                obfuscated: obj
                    .get("obfuscated")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(parent_style.obfuscated),
            };

            if let Some(t) = obj.get("text").and_then(|v| v.as_str()) {
                if !t.is_empty() {
                    let parsed = McText::parse(t);
                    if parsed.is_empty() || parsed.spans().iter().all(|s| s.color.is_none()) {
                        text.push(Span {
                            text: t.to_string(),
                            color,
                            style,
                        });
                    } else {
                        for mut span in parsed.into_spans() {
                            if span.color.is_none() {
                                span.color = color;
                            }
                            span.style = span.style.merge(&style);
                            text.push(span);
                        }
                    }
                }
            }

            if let Some(translate) = obj.get("translate").and_then(|v| v.as_str()) {
                text.push(Span {
                    text: translate.to_string(),
                    color,
                    style,
                });
            }

            if let Some(extra) = obj.get("extra").and_then(|v| v.as_array()) {
                for item in extra {
                    extract_spans(item, color, style, text);
                }
            }
        }
        Value::Array(arr) => {
            for item in arr {
                extract_spans(item, parent_color, parent_style, text);
            }
        }
        _ => {}
    }
}

pub fn to_json(text: &McText) -> String {
    if text.spans().is_empty() {
        return r#"{"text":""}"#.to_string();
    }

    if text.spans().len() == 1 {
        let span = &text.spans()[0];
        return span_to_json(span);
    }

    let mut components: Vec<String> = Vec::new();
    components.push(r#"{"text":""}"#.to_string());

    for span in text.spans() {
        components.push(span_to_json(span));
    }

    format!("[{}]", components.join(","))
}

fn span_to_json(span: &Span) -> String {
    let mut parts = Vec::new();

    let escaped_text = span
        .text
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n");
    parts.push(format!(r#""text":"{}""#, escaped_text));

    if let Some(color) = span.color {
        let color_str = match color {
            TextColor::Named(named) => named.name().to_string(),
            TextColor::Rgb { r, g, b } => format!("#{:02x}{:02x}{:02x}", r, g, b),
        };
        parts.push(format!(r#""color":"{}""#, color_str));
    }

    if span.style.bold {
        parts.push(r#""bold":true"#.to_string());
    }
    if span.style.italic {
        parts.push(r#""italic":true"#.to_string());
    }
    if span.style.underlined {
        parts.push(r#""underlined":true"#.to_string());
    }
    if span.style.strikethrough {
        parts.push(r#""strikethrough":true"#.to_string());
    }
    if span.style.obfuscated {
        parts.push(r#""obfuscated":true"#.to_string());
    }

    format!("{{{}}}", parts.join(","))
}

pub fn to_legacy(text: &McText) -> String {
    text.to_legacy()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::NamedColor;

    #[test]
    fn test_parse_simple_text() {
        let json = r#"{"text":"Hello World"}"#;
        let text = parse_json_component(json);
        assert_eq!(text.plain_text(), "Hello World");
    }

    #[test]
    fn test_parse_colored_text() {
        let json = r#"{"text":"Gold","color":"gold"}"#;
        let text = parse_json_component(json);
        assert_eq!(
            text.spans()[0].color,
            Some(TextColor::Named(NamedColor::Gold))
        );
    }

    #[test]
    fn test_parse_hex_color() {
        let json = r##"{"text":"Custom","color":"#FF5500"}"##;
        let text = parse_json_component(json);
        assert_eq!(
            text.spans()[0].color,
            Some(TextColor::Rgb {
                r: 255,
                g: 85,
                b: 0
            })
        );
    }

    #[test]
    fn test_parse_styled_text() {
        let json = r#"{"text":"Bold","bold":true,"italic":true}"#;
        let text = parse_json_component(json);
        assert!(text.spans()[0].style.bold);
        assert!(text.spans()[0].style.italic);
    }

    #[test]
    fn test_parse_extra() {
        let json = r#"{"text":"","extra":[{"text":"Hello ","color":"gold"},{"text":"World","color":"aqua"}]}"#;
        let text = parse_json_component(json);
        assert_eq!(text.spans().len(), 2);
        assert_eq!(text.spans()[0].text, "Hello ");
        assert_eq!(
            text.spans()[0].color,
            Some(TextColor::Named(NamedColor::Gold))
        );
        assert_eq!(text.spans()[1].text, "World");
        assert_eq!(
            text.spans()[1].color,
            Some(TextColor::Named(NamedColor::Aqua))
        );
    }

    #[test]
    fn test_parse_string_value() {
        let json = r#""Hello World""#;
        let text = parse_json_component(json);
        assert_eq!(text.plain_text(), "Hello World");
    }

    #[test]
    fn test_to_json_named_color() {
        let mut text = McText::new();
        text.push(Span::new("Hello").with_color(NamedColor::Gold));
        let json = to_json(&text);
        assert!(json.contains("gold"));
        assert!(json.contains("Hello"));
    }

    #[test]
    fn test_to_json_rgb_color() {
        let mut text = McText::new();
        text.push(Span::new("Custom").with_color(TextColor::Rgb {
            r: 255,
            g: 128,
            b: 0,
        }));
        let json = to_json(&text);
        assert!(json.contains("#ff8000"));
    }

    #[test]
    fn test_parse_nested_legacy() {
        let json = r#"{"text":"\u00A76Gold \u00A7bAqua"}"#;
        let text = parse_json_component(json);
        assert_eq!(text.plain_text(), "Gold Aqua");
    }
}
