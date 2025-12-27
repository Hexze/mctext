# mctext

Minecraft text formatting, parsing, and rendering.

![Font Showcase](https://raw.githubusercontent.com/hexze/mctext/master/showcase.png)

## Features

- **Text Parsing**: Parse legacy formatting codes and JSON chat components
- **Color Support**: All 16 named Minecraft colors plus RGB hex colors
- **Style Handling**: Bold, italic, underlined, strikethrough, obfuscated
- **Font Rendering**: Measure and render text with Minecraft fonts (modern + legacy)

## Usage

```toml
[dependencies]
mctext = "0.1"

# For font rendering:
mctext = { version = "0.1", features = ["render"] }
```

### Parsing Text

```rust
use mctext::{McText, TextColor, NamedColor};

let text = McText::parse("§cRed §lBold");

for span in text.spans() {
    println!("{}: {:?}", span.text, span.color);
}
```

### JSON Chat Components

```rust
use mctext::{parse_json_component, try_parse_json_component};

let json = r#"{"text":"Hello","color":"gold","bold":true}"#;

// Returns empty McText on error
let text = parse_json_component(json);

// Returns Result with error details
let text = try_parse_json_component(json)?;
```

### Rendering (requires `render` feature)

```rust
use mctext::{FontSystem, TextRenderContext, LayoutOptions, SoftwareRenderer};

let font_system = FontSystem::modern();
let ctx = TextRenderContext::new(&font_system);
let mut renderer = SoftwareRenderer::new(&font_system, 200, 50);

ctx.render_str(
    &mut renderer,
    "§6Gold Text",
    10.0, 20.0,
    &LayoutOptions::new(16.0)
)?;
```

## Feature Flags

| Feature | Description |
|---------|-------------|
| `modern-fonts` | Modern Minecraft fonts (default) |
| `legacy-fonts` | Pre-1.13 classic fonts |
| `special-fonts` | Enchanting and Illager fonts |
| `all-fonts` | All font variants |
| `serde` | Serialization support |

```toml
# Modern fonts only (default)
mctext = "1.0"

# With legacy fonts
mctext = { version = "1.0", features = ["legacy-fonts"] }

# All fonts
mctext = { version = "1.0", features = ["all-fonts"] }
```

## Font Versions

```rust
use mctext::FontSystem;

let modern = FontSystem::modern();  // requires modern-fonts feature
let legacy = FontSystem::legacy();  // requires legacy-fonts feature
```

## License

MIT
