use crate::color::TextColor;
use crate::style::Style;
use crate::text::{MCText, Span};
use serde_json::{Map, Value};
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

pub fn try_parse_json_component(json: &str) -> Result<MCText, ParseError> {
    let value =
        serde_json::from_str::<Value>(json).map_err(|e| ParseError::InvalidJson(e.to_string()))?;
    Ok(parse_value(&value))
}

fn parse_value(value: &Value) -> MCText {
    let mut text = MCText::new();
    extract_spans(value, None, Style::default(), &mut text);
    text
}

fn extract_color(obj: &Map<String, Value>, fallback: Option<TextColor>) -> Option<TextColor> {
    obj.get("color")
        .and_then(|v| v.as_str())
        .and_then(TextColor::parse)
        .or(fallback)
}

fn extract_style(obj: &Map<String, Value>, parent: &Style) -> Style {
    let get_bool = |key: &str, default: bool| -> bool {
        obj.get(key).and_then(|v| v.as_bool()).unwrap_or(default)
    };

    Style {
        bold: get_bool("bold", parent.bold),
        italic: get_bool("italic", parent.italic),
        underlined: get_bool("underlined", parent.underlined),
        strikethrough: get_bool("strikethrough", parent.strikethrough),
        obfuscated: get_bool("obfuscated", parent.obfuscated),
    }
}

fn push_text_with_inheritance(
    content: &str,
    color: Option<TextColor>,
    style: Style,
    text: &mut MCText,
) {
    if content.is_empty() {
        return;
    }

    let parsed = MCText::parse(content);
    if parsed.is_empty() {
        text.push(Span {
            text: content.to_string(),
            color,
            style,
        });
        return;
    }

    let has_color_codes = parsed.spans().iter().any(|s| s.color.is_some());
    if !has_color_codes {
        text.push(Span {
            text: content.to_string(),
            color,
            style,
        });
        return;
    }

    for mut span in parsed.into_spans() {
        if span.color.is_none() {
            span.color = color;
        }
        span.style = span.style.merge(&style);
        text.push(span);
    }
}

fn extract_spans(
    value: &Value,
    parent_color: Option<TextColor>,
    parent_style: Style,
    text: &mut MCText,
) {
    match value {
        Value::String(s) => {
            push_text_with_inheritance(s, parent_color, parent_style, text);
        }
        Value::Object(obj) => {
            let color = extract_color(obj, parent_color);
            let style = extract_style(obj, &parent_style);

            if let Some(t) = obj.get("text").and_then(|v| v.as_str()) {
                push_text_with_inheritance(t, color, style, text);
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

pub fn to_json(text: &MCText) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NamedColor;

    #[test]
    fn test_parse_json() {
        let json = r#"{"text":"","extra":[{"text":"Hello ","color":"gold"},{"text":"World","color":"aqua"}]}"#;
        let text = try_parse_json_component(json).unwrap();
        assert_eq!(text.plain_text(), "Hello World");
        assert_eq!(text.spans().len(), 2);
    }

    #[test]
    fn test_to_json() {
        let mut text = MCText::new();
        text.push(Span::new("Hello").with_color(NamedColor::Gold));
        let json = to_json(&text);
        assert!(json.contains("gold") && json.contains("Hello"));
    }
}
