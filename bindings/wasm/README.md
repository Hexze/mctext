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

## Fonts Only

Looking for just the TTF files? Download them from the [releases page](https://github.com/hexze/mctext/releases):

- `minecraft-fonts-modern.zip` - Latest Minecraft fonts (updated, cleaner look)
- `minecraft-fonts-legacy.zip` - Classic fonts for those who prefer pre-1.13
- `minecraft-fonts-special.zip` - Enchanting and Illager fonts

## Usage

### Rust

```toml
[dependencies]
mctext = "1.0"

# With legacy fonts:
mctext = { version = "1.0", features = ["legacy-fonts"] }
```

```rust
use mctext::{MCText, NamedColor};

let text = MCText::new()
    .span("Red ").color(NamedColor::Red)
    .then("Bold").color(NamedColor::Red).bold()
    .build();

for span in text.spans() {
    println!("{}: {:?}", span.text, span.color);
}
```

### Python

```bash
pip install mctext
```

```python
import mctext

text = mctext.MCText().span("Red ").color("red").then("Bold").color("red").bold().build()

for span in text.spans():
    print(f"{span.text}: {span.color}")
```

### JavaScript

```bash
npm install @hexze/mctext
```

```javascript
import init, { MCText } from '@hexze/mctext';

await init();

let text = new MCText().span("Red ").color("red").then("Bold").color("red").bold().build();

for (const span of text.spans()) {
    console.log(`${span.text}: ${span.color}`);
}
```

## License

MIT
