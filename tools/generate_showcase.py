import json
from pathlib import Path

from PIL import Image, ImageDraw

import mctext

SCRIPT_DIR = Path(__file__).parent
PROJECT_DIR = SCRIPT_DIR.parent
CACHE_DIR = SCRIPT_DIR / ".cache"


def get_latest_version() -> str:
    manifest = CACHE_DIR / "manifest.json"
    if manifest.exists():
        with open(manifest, encoding="utf-8") as f:
            data = json.load(f)
            return data["latest"]["release"]
    return "1.21"


def render_text(font_system, text: str, size: float, width: int, height: int) -> Image.Image:
    opts = mctext.LayoutOptions(size, shadow=False)
    result = mctext.render(font_system, text, width, height, opts)
    return Image.frombytes("RGBA", (result.width, result.height), bytes(result.to_bytes()))


def render_family(font_system, text: str, size: float, width: int, height: int, family) -> Image.Image:
    result = mctext.render_family(font_system, text, width, height, size, family)
    return Image.frombytes("RGBA", (result.width, result.height), bytes(result.to_bytes()))


def colorize(img: Image.Image, color: tuple[int, int, int]) -> Image.Image:
    data = img.getdata()
    new_data = []
    for r, g, b, a in data:
        if a > 0:
            new_data.append((color[0], color[1], color[2], a))
        else:
            new_data.append((0, 0, 0, 0))
    result = Image.new("RGBA", img.size)
    result.putdata(new_data)
    return result


def main():
    create_showcase()


def create_showcase():
    padding = 25
    col_width = 280
    size = 24
    line_height = 28

    num_lines = 5
    box_height = num_lines * line_height

    img_width = padding * 4 + col_width * 3
    img_height = padding * 2 + box_height

    img = Image.new("RGBA", (img_width, img_height), (24, 24, 24, 255))
    draw = ImageDraw.Draw(img)

    modern_fs = mctext.FontSystem.modern()
    legacy_fs = mctext.FontSystem.legacy()
    version = get_latest_version()

    def paste_text(fs, text, x, y, w=col_width, h=line_height, sz=size):
        rendered = render_text(fs, text, sz, w, h)
        img.paste(rendered, (x, y), rendered)

    box_x = padding
    y = padding
    paste_text(modern_fs, f"\u00a77Modern ({version})", box_x, y)
    y += line_height
    paste_text(modern_fs, "\u00a7aThe quick brown fox", box_x, y)
    y += line_height
    paste_text(modern_fs, "\u00a7c你好世界  \u00a76こんにちは", box_x, y)
    y += line_height
    paste_text(modern_fs, "\u00a79Привет мир  \u00a7dΓειά σου", box_x, y)
    y += line_height
    paste_text(modern_fs, "\u00a7e?  !  |  ()  {}  :", box_x, y)

    box_x = padding * 2 + col_width
    y = padding
    paste_text(legacy_fs, "\u00a77Legacy (1.8.9)", box_x, y)
    y += line_height
    paste_text(legacy_fs, "\u00a7aThe quick brown fox", box_x, y)
    y += line_height
    paste_text(legacy_fs, "\u00a7c你好世界  \u00a76こんにちは", box_x, y)
    y += line_height
    paste_text(legacy_fs, "\u00a79Привет мир  \u00a7dΓειά σου", box_x, y)
    y += line_height
    paste_text(legacy_fs, "\u00a7e?  !  |  ()  {}  :", box_x, y)

    right_col_x = padding * 3 + col_width * 2
    sample_text = "abcdefghij"

    y = padding
    paste_text(modern_fs, "\u00a77Illager", right_col_x, y)
    illager_width = int(modern_fs.measure_family(sample_text, size, mctext.FontFamily.Illager))
    illager_img = render_family(modern_fs, sample_text, size, illager_width + 5, line_height, mctext.FontFamily.Illager)
    illager_colored = colorize(illager_img, (255, 85, 85))
    illager_x = right_col_x + col_width - illager_width - 5
    img.paste(illager_colored, (illager_x, y), illager_colored)

    y += line_height
    paste_text(modern_fs, "\u00a77Enchanting", right_col_x, y)
    enchanting_width = int(modern_fs.measure_family(sample_text, size, mctext.FontFamily.Enchanting))
    enchanting_img = render_family(modern_fs, sample_text, size, enchanting_width + 5, line_height, mctext.FontFamily.Enchanting)
    enchanting_colored = colorize(enchanting_img, (85, 255, 85))
    enchanting_x = right_col_x + col_width - enchanting_width - 5
    img.paste(enchanting_colored, (enchanting_x, y), enchanting_colored)

    metrics_y_start = y + line_height * 2 + 6
    metrics_size = 48
    sample = "Quickly"

    ascent_px = int(metrics_size * 0.75)
    xheight_px = int(metrics_size * 0.53)
    descent_px = int(metrics_size * 0.167)

    cap_y = metrics_y_start
    xheight_y = cap_y + (ascent_px - xheight_px)
    baseline_y = cap_y + ascent_px
    descent_y = baseline_y + descent_px

    metrics = [
        (cap_y, (255, 85, 85), "cap"),
        (xheight_y, (255, 170, 0), "x-height"),
        (baseline_y, (85, 255, 85), "baseline"),
        (descent_y, (85, 85, 255), "descent"),
    ]

    text_width = int(modern_fs.measure(sample, metrics_size))
    line_start = right_col_x
    line_end = right_col_x + col_width - 60
    for my, color, label in metrics:
        draw.line([(line_start, my), (line_end, my)], fill=color, width=1)

    text_x = right_col_x
    rendered = render_text(modern_fs, sample, metrics_size, text_width + 5, int(metrics_size * 1.2))
    img.paste(rendered, (text_x, cap_y), rendered)

    label_size = 11
    label_x = line_end + 4
    for my, color, label in metrics:
        color_code = {
            (255, 85, 85): "\u00a7c",
            (255, 170, 0): "\u00a76",
            (85, 255, 85): "\u00a7a",
            (85, 85, 255): "\u00a79",
        }.get(color, "\u00a7f")
        lbl = render_text(modern_fs, f"{color_code}{label}", label_size, 55, 14)
        img.paste(lbl, (label_x, my - 7), lbl)

    output = PROJECT_DIR / "showcase.png"
    img.save(output)
    print(f"Generated {output}")


if __name__ == "__main__":
    main()
