# mctext

Minecraft text formatting, parsing, and rendering library. Features all the exact glyphs used ingame, for the fonts of both pre and post-1.13.

## Language Support

| Language | Package | Registry |
|----------|---------|----------|
| Rust | `mctext` | [crates.io](https://crates.io/crates/mctext) |
| Python | `mctext` | [PyPI](https://pypi.org/project/mctext) |
| JavaScript | `@hexze/mctext` | [npm](https://npmjs.com/package/@hexze/mctext) |

## Features

- **Builder API** - Fluent interface for constructing formatted text
- **Color Support** - All 16 named Minecraft colors plus RGB hex colors
- **Style Handling** - Bold, italic, underlined, strikethrough, obfuscated
- **Font Rendering** - Measure and render text with authentic Minecraft fonts
- **Legacy Support** - Parse `§` formatting codes and JSON chat components

## Font Showcase

![Font Showcase](https://raw.githubusercontent.com/hexze/mctext/master/showcase.png)

## Installation

```bash
pip install mctext
```

## Quick Start

```python
import mctext

text = mctext.MCText().span("Red ").color("red").then("Bold").color("red").bold().build()

for span in text.spans():
    print(f"{span.text}: {span.color}")
```

## API Reference

### MCText

| Method | Description |
|--------|-------------|
| `MCText()` | Create empty MCText |
| `MCText.parse(text)` | Parse legacy `§` formatted text |
| `MCText.parse_json(json)` | Parse JSON chat component |
| `span(text)` | Start building a span, returns SpanBuilder |
| `spans()` | Get list of text spans |
| `plain_text()` | Get text without formatting |
| `to_legacy()` | Convert to legacy `§` format |
| `to_json()` | Convert to JSON chat component |

### SpanBuilder

| Method | Description |
|--------|-------------|
| `color(color)` | Set span color (name like "red" or hex "#FF0000") |
| `bold()` | Make span bold |
| `italic()` | Make span italic |
| `underlined()` | Make span underlined |
| `strikethrough()` | Make span strikethrough |
| `obfuscated()` | Make span obfuscated |
| `then(text)` | Add another span and continue building |
| `build()` | Finish building and return MCText |

### Span

| Property | Type | Description |
|----------|------|-------------|
| `text` | str | The text content |
| `color` | Color | Color (named or RGB) |
| `style` | Style | Formatting flags |

### Color

| Property | Type | Description |
|----------|------|-------------|
| `name` | str | Color name (named colors only) |
| `code` | str | Format code character (named colors only) |
| `rgb` | tuple | RGB tuple (r, g, b) |
| `r`, `g`, `b` | int | Individual components |

### Style

| Property | Type | Description |
|----------|------|-------------|
| `bold` | bool | Bold text |
| `italic` | bool | Italic text |
| `underlined` | bool | Underlined text |
| `strikethrough` | bool | Strikethrough text |
| `obfuscated` | bool | Randomized text |

### Utility Functions

| Function | Description |
|----------|-------------|
| `parse(text)` | Shorthand for `MCText.parse(text)` |
| `parse_json(json)` | Shorthand for `MCText.parse_json(json)` |
| `strip_codes(text)` | Remove all `§` codes from text |
| `count_visible_chars(text)` | Count characters excluding format codes |
| `named_colors()` | Get all 16 named Minecraft colors |

## Rendering

```python
import mctext

text = mctext.MCText().span("Hello ").color("red").then("World").color("red").bold().build()

fonts = mctext.FontSystem.modern()
options = mctext.LayoutOptions(16.0, None, True)  # size, max_width, shadow
result = mctext.render(fonts, text, 256, 64, options)

# result.width, result.height - dimensions
# result.data() - RGBA bytes
```

### Rendering API

| Class/Function | Description |
|----------------|-------------|
| `FontSystem.modern()` | Load post-1.13 fonts |
| `FontSystem.legacy()` | Load pre-1.13 fonts |
| `FontSystem.measure(text, size)` | Measure text width in pixels |
| `LayoutOptions(size, max_width, shadow)` | Layout options |
| `render(fonts, text, width, height, options)` | Render to RGBA buffer |

## License

MIT
