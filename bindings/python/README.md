# mctext

Minecraft text formatting, parsing, and rendering for Python.

![Font Showcase](https://raw.githubusercontent.com/hexze/mctext/master/showcase.png)

## Installation

```bash
pip install mctext
```

## Usage

```python
import mctext

# Parse legacy format codes
text = mctext.parse("§cRed §lBold")
for span in text.spans():
    print(f"{span.text}: {span.color.rgb}")

# Parse JSON chat components
text = mctext.parse_json('{"text":"Hello","color":"gold"}')
```

## License

MIT
