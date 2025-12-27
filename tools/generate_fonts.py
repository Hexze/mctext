"""
Generate Minecraft TTF fonts from JAR files.

Extracts bitmap fonts from Minecraft's JAR and converts them to TrueType format
using Moore-neighborhood edge-following for vectorization.

Adapted from https://github.com/tryashtar/minecraft-ttf

Usage:
    python generate_fonts.py                # Generate both modern and legacy
    python generate_fonts.py --modern       # Only modern fonts
    python generate_fonts.py --legacy       # Only legacy fonts
    python generate_fonts.py --version X    # Use specific version for modern
"""

import argparse
import datetime
import io
import json
import os
import re
import zipfile
from pathlib import Path

import PIL.Image
import requests

os.environ["PYGAME_HIDE_SUPPORT_PROMPT"] = "1"
import pygame
import fontTools.fontBuilder
import fontTools.pens.ttGlyphPen
import fontTools.ttLib.tables._g_l_y_f

SCRIPT_DIR = Path(__file__).parent
PROJECT_DIR = SCRIPT_DIR.parent
ASSETS_DIR = PROJECT_DIR / "crates" / "mctext" / "assets"
CACHE_DIR = SCRIPT_DIR / ".cache"

LEGACY_VERSION = "1.8.9"

FONT_EM = 1200
PIXEL_SCALE = FONT_EM / 12


def get_version_manifest() -> dict:
    cached = CACHE_DIR / "manifest.json"
    if cached.exists():
        with open(cached, encoding="utf-8") as f:
            return json.load(f)

    print("Downloading version manifest...")
    url = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json"
    data = requests.get(url).json()
    CACHE_DIR.mkdir(parents=True, exist_ok=True)
    with open(cached, "w", encoding="utf-8") as f:
        json.dump(data, f)
    return data


def get_latest_release() -> str:
    manifest = get_version_manifest()
    return manifest["latest"]["release"]


def get_version_meta(version: str) -> dict:
    manifest = get_version_manifest()
    for v in manifest["versions"]:
        if v["id"] == version:
            return requests.get(v["url"]).json()
    raise ValueError(f"Version {version} not found")


def download_jar(version: str) -> Path:
    cached = CACHE_DIR / f"minecraft-{version}.jar"
    if cached.exists():
        return cached

    print(f"Downloading Minecraft {version}...")
    meta = get_version_meta(version)
    url = meta["downloads"]["client"]["url"]

    CACHE_DIR.mkdir(parents=True, exist_ok=True)
    with open(cached, "wb") as f:
        for chunk in requests.get(url, stream=True).iter_content(16384):
            f.write(chunk)
    return cached


def download_asset(version: str, path: str) -> Path:
    cached = CACHE_DIR / path.replace("/", "_")
    if cached.exists():
        return cached

    meta = get_version_meta(version)
    index = requests.get(meta["assetIndex"]["url"]).json()

    if path not in index["objects"]:
        raise ValueError(f"Asset not found: {path}")

    hash = index["objects"][path]["hash"]
    url = f"https://resources.download.minecraft.net/{hash[:2]}/{hash}"

    print(f"  Downloading {path}...")
    CACHE_DIR.mkdir(parents=True, exist_ok=True)
    with open(cached, "wb") as f:
        f.write(requests.get(url).content)
    return cached


def get_aglfn() -> dict:
    cached = CACHE_DIR / "aglfn.txt"
    if not cached.exists():
        print("Downloading Adobe AGLFN...")
        url = "https://raw.githubusercontent.com/adobe-type-tools/agl-aglfn/master/aglfn.txt"
        CACHE_DIR.mkdir(parents=True, exist_ok=True)
        with open(cached, "wb") as f:
            f.write(requests.get(url).content)

    result = {}
    with open(cached, encoding="utf-8") as f:
        for line in f:
            if line.startswith("#") or not line.strip():
                continue
            parts = line.strip().split(";")
            if len(parts) >= 2:
                result[chr(int(parts[0], 16))] = parts[1]
    return result


def parse_minecraft_json(text: str) -> dict:
    return json.loads(re.sub(r",(\s*[}\]])", r"\1", text))


def parse_unihex_zip(path: Path) -> dict:
    glyphs = {}
    with zipfile.ZipFile(path) as z:
        for name in z.namelist():
            if not name.endswith(".hex"):
                continue
            for line in z.read(name).decode("utf-8").splitlines():
                if ":" not in line:
                    continue
                codepoint, data = line.strip().split(":", 1)
                char = chr(int(codepoint, 16))

                width = len(data) // 4
                bytes_per_row = width // 8

                rows = []
                for i in range(16):
                    start = i * bytes_per_row * 2
                    end = start + bytes_per_row * 2
                    bits = int(data[start:end], 16) if data[start:end] else 0
                    rows.append((width, bits))
                glyphs[char] = (width, rows)
    return glyphs


def unihex_to_mask(width: int, rows: list) -> pygame.mask.Mask:
    mask = pygame.mask.Mask((width, 16), fill=False)
    for y, (w, bits) in enumerate(rows):
        for x in range(w):
            if bits & (1 << (w - 1 - x)):
                mask.set_at((x, y), 1)
    return mask


class FontBuilder:
    def __init__(self):
        self.seen = set()
        self.fonts = {s: {} for s in ["Regular", "Bold", "Italic", "Bold Italic"]}

    def add_space(self, char: str, width: float):
        if char in self.seen:
            return
        self.seen.add(char)
        self.fonts["Regular"][char] = {"width": width * PIXEL_SCALE, "path": None}
        self.fonts["Italic"][char] = {"width": width * PIXEL_SCALE, "path": None}
        self.fonts["Bold"][char] = {"width": (width + 1) * PIXEL_SCALE, "path": None}
        self.fonts["Bold Italic"][char] = {"width": (width + 1) * PIXEL_SCALE, "path": None}

    def add_glyph(self, char: str, mask: pygame.mask.Mask, height: int, ascent: int):
        if char in self.seen:
            return
        self.seen.add(char)

        w, h = mask.get_size()
        scale = height / h * PIXEL_SCALE
        add_width = h / height

        bold = pygame.mask.Mask((w + 1, h), fill=False)
        bold.draw(mask, (0, 0))
        bold.draw(mask, (1, 0))

        offset = (0, (height - ascent) / height * h)
        italic_offset = (-6 / height, offset[1])

        for style, m, off in [
            ("Regular", mask, offset),
            ("Italic", mask, italic_offset),
            ("Bold", bold, offset),
            ("Bold Italic", bold, italic_offset),
        ]:
            italic = "Italic" in style
            path, (gw, gh) = vectorize(m, scale, off, italic)
            self.fonts[style][char] = {
                "width": (gw + add_width) * scale,
                "height": gh * scale,
                "path": path,
            }

    def save(self, name: str, output_dir: Path, aglfn: dict, dates: tuple):
        suffixes = {"Regular": "", "Bold": "-bold", "Italic": "-italic", "Bold Italic": "-bold-italic"}
        for style, data in self.fonts.items():
            filename = f"{name}{suffixes[style]}.ttf"
            font = build_ttf(f"Minecraft {name.title()}", style, dates, data, aglfn)
            font.save(str(output_dir / filename))
            print(f"    {filename}")


def generate_modern_fonts(version: str, aglfn: dict):
    jar_path = download_jar(version)
    output = ASSETS_DIR / "modern"
    output.mkdir(parents=True, exist_ok=True)

    with zipfile.ZipFile(jar_path) as jar:
        fonts = [
            ("minecraft", "assets/minecraft/font/default.json"),
            ("enchanting", "assets/minecraft/font/alt.json"),
            ("illager", "assets/minecraft/font/illageralt.json"),
        ]
        for name, entry in fonts:
            print(f"  Converting {name}...")
            builder = FontBuilder()
            process_modern_font(jar, entry, builder, version)

            created = datetime.datetime(2009, 5, 16, tzinfo=datetime.timezone.utc)
            modified = datetime.datetime.now(datetime.timezone.utc)
            builder.save(name, output, aglfn, (created, modified))


def process_modern_font(jar: zipfile.ZipFile, entry: str, builder: FontBuilder, version: str):
    data = json.loads(jar.read(entry))

    providers = list(data["providers"])
    i = 0
    while i < len(providers):
        if providers[i]["type"] == "reference":
            ref_id = providers[i]["id"]
            ns, path = ref_id.split(":")
            ref_path = f"assets/{ns}/font/{path}.json"
            ref_data = json.loads(jar.read(ref_path))

            if not ref_data["providers"]:
                asset = download_asset(version, f"{ns}/font/{path}.json")
                with open(asset, encoding="utf-8") as f:
                    ref_data = parse_minecraft_json(f.read())

            del providers[i]
            providers[i:i] = ref_data["providers"]
        else:
            i += 1

    notdef = pygame.mask.Mask((5, 8), fill=False)
    for y in range(8):
        for x in range(5):
            if x == 0 or y == 0 or x == 4 or y == 7:
                notdef.set_at((x, y), 1)
    builder.add_glyph(".notdef", notdef, 8, 8)

    for provider in providers:
        ptype = provider["type"]

        if ptype == "space":
            for char, width in provider["advances"].items():
                builder.add_space(char, width)

        elif ptype == "bitmap":
            try:
                ns, path = provider["file"].split(":")
                img_data = jar.read(f"assets/{ns}/textures/{path}")
                img = PIL.Image.open(io.BytesIO(img_data)).convert("RGBA")
            except KeyError:
                continue

            height = provider.get("height", 8)
            ascent = provider["ascent"]
            chars = provider["chars"]
            gw = img.width // len(chars[0])
            gh = img.height // len(chars)

            for y, row in enumerate(chars):
                for x, char in enumerate(row):
                    if char == "\0":
                        continue
                    glyph = img.crop((x * gw, y * gh, (x + 1) * gw, (y + 1) * gh))
                    surface = pygame.image.fromstring(glyph.tobytes(), glyph.size, "RGBA")
                    mask = pygame.mask.from_surface(surface)
                    builder.add_glyph(char, mask, height, ascent)

        elif ptype == "unihex":
            ns, path = provider["hex_file"].split(":")
            try:
                hex_path = download_asset(version, f"{ns}/{path}")
                glyphs = parse_unihex_zip(hex_path)
                glyphs = {c: v for c, v in glyphs.items() if ord(c) <= 0xFFFF}
                print(f"    Loaded {len(glyphs):,} glyphs from {path}")

                for char, (width, rows) in glyphs.items():
                    mask = unihex_to_mask(width, rows)
                    builder.add_glyph(char, mask, 8, 7)
            except Exception as e:
                print(f"    Warning: {e}")


def generate_legacy_fonts(aglfn: dict):
    jar_path = download_jar(LEGACY_VERSION)
    output = ASSETS_DIR / "legacy"
    output.mkdir(parents=True, exist_ok=True)

    print("  Converting minecraft...")
    builder = FontBuilder()

    with zipfile.ZipFile(jar_path) as jar:
        notdef = pygame.mask.Mask((5, 8), fill=False)
        for y in range(8):
            for x in range(5):
                if x == 0 or y == 0 or x == 4 or y == 7:
                    notdef.set_at((x, y), 1)
        builder.add_glyph(".notdef", notdef, 8, 8)
        builder.add_space(" ", 4)

        ascii_data = jar.read("assets/minecraft/textures/font/ascii.png")
        ascii_img = PIL.Image.open(io.BytesIO(ascii_data)).convert("RGBA")
        gw, gh = ascii_img.width // 16, ascii_img.height // 16

        for i in range(256):
            if i == 32:
                continue
            x, y = i % 16, i // 16
            glyph = ascii_img.crop((x * gw, y * gh, (x + 1) * gw, (y + 1) * gh))
            surface = pygame.image.fromstring(glyph.tobytes(), glyph.size, "RGBA")
            mask = pygame.mask.from_surface(surface)
            if mask.count() > 0:
                builder.add_glyph(chr(i), mask, 8, 7)

        print(f"    Loaded {len(builder.seen)} ASCII characters")

        glyph_sizes = None
        try:
            glyph_sizes = jar.read("assets/minecraft/font/glyph_sizes.bin")
        except KeyError:
            pass

        unicode_count = 0
        for page in range(1, 256):
            if 0xD8 <= page <= 0xDF:
                continue
            try:
                path = f"assets/minecraft/textures/font/unicode_page_{page:02x}.png"
                img = PIL.Image.open(io.BytesIO(jar.read(path))).convert("RGBA")
            except KeyError:
                continue

            gw, gh = img.width // 16, img.height // 16
            for i in range(256):
                cp = page * 256 + i
                char = chr(cp)
                if char in builder.seen:
                    continue

                x, y = i % 16, i // 16
                glyph = img.crop((x * gw, y * gh, (x + 1) * gw, (y + 1) * gh))

                if glyph_sizes and cp < len(glyph_sizes):
                    byte = glyph_sizes[cp]
                    start, end = (byte >> 4) & 0xF, byte & 0xF
                    if end > start:
                        sx = int(start * gw / 16)
                        ex = int((end + 1) * gw / 16)
                        glyph = img.crop((x * gw + sx, y * gh, x * gw + ex, (y + 1) * gh))

                surface = pygame.image.fromstring(glyph.tobytes(), glyph.size, "RGBA")
                mask = pygame.mask.from_surface(surface)
                if mask.count() > 0:
                    builder.add_glyph(char, mask, 8, 7)
                    unicode_count += 1

        print(f"    Loaded {unicode_count} Unicode characters")

    created = datetime.datetime(2009, 5, 16, tzinfo=datetime.timezone.utc)
    modified = datetime.datetime.now(datetime.timezone.utc)
    builder.save("minecraft", output, aglfn, (created, modified))


def build_ttf(name: str, style: str, dates: tuple, glyphs: dict, aglfn: dict):
    empty = fontTools.ttLib.tables._g_l_y_f.Glyph()

    order = [".notdef", ".null"]
    cmap = {}
    widths = {".notdef": 0, ".null": 0}
    paths = {".notdef": empty, ".null": empty}

    for char, data in glyphs.items():
        if char in (".notdef", ".null"):
            glyph_name = char
        else:
            glyph_name = aglfn.get(char, f"uni{ord(char):04X}")
            order.append(glyph_name)
            cmap[ord(char)] = glyph_name

        widths[glyph_name] = data["width"]
        paths[glyph_name] = data["path"] if data["path"] else empty

    max_width = max((d["width"] for d in glyphs.values()), default=0)
    max_height = max((d.get("height", 0) for d in glyphs.values()), default=0)

    font = fontTools.fontBuilder.FontBuilder(unitsPerEm=FONT_EM, isTTF=True)
    font.setupGlyphOrder(order)
    font.setupCharacterMap(cmap)
    font.setupGlyf(paths)

    glyf = font.font["glyf"]
    font.setupHorizontalMetrics({n: (w, glyf[n].xMin) for n, w in widths.items()})

    ascent = FONT_EM * 9 // 12
    descent = FONT_EM * 2 // 12
    font.setupHorizontalHeader(ascent=ascent, descent=-descent)

    font.setupNameTable({
        "copyright": "Copyright (c) Mojang AB",
        "familyName": name,
        "styleName": style,
        "uniqueFontIdentifier": f"{name.replace(' ', '')}.{style.replace(' ', '')}",
        "fullName": f"{name} {style}",
        "version": "Version 1.0",
        "psName": f"{name.replace(' ', '')}{style.replace(' ', '')}",
    })

    weight = 700 if "Bold" in style else 400
    mac = (1 if "Bold" in style else 0) + (2 if "Italic" in style else 0)
    fs = (32 if "Bold" in style else 0) + (1 if "Italic" in style else 0)
    if not mac:
        fs = 64

    font.setupOS2(
        sTypoAscender=ascent,
        sTypoDescender=-descent,
        usWinAscent=ascent,
        usWinDescent=descent,
        sCapHeight=FONT_EM * 7 // 12,
        sxHeight=FONT_EM * 5 // 12,
        yStrikeoutPosition=FONT_EM * 4 // 12,
        yStrikeoutSize=FONT_EM // 12,
        sTypoLineGap=0,
        fsSelection=fs,
        achVendID="",
        usWeightClass=weight,
    )

    font.setupPost(
        underlinePosition=-FONT_EM // 12,
        underlineThickness=FONT_EM // 12,
        italicAngle=-14.05598 if "Italic" in style else 0,
    )

    epoch = datetime.datetime(1904, 1, 1, tzinfo=datetime.timezone.utc)
    font.updateHead(
        xMin=0,
        xMax=int(max_width),
        yMin=-descent,
        yMax=int(max_height),
        created=int((dates[0] - epoch).total_seconds()),
        modified=int((dates[1] - epoch).total_seconds()),
        macStyle=mac,
    )

    return font


def vectorize(mask: pygame.mask.Mask, scale: float, offset: tuple, italic: bool = False):
    w, h = mask.get_size()
    ox, oy = offset

    pen = fontTools.pens.ttGlyphPen.TTGlyphPen(None)

    def transform(x, y):
        x += ox
        y += oy
        if italic:
            x += (h - y) / 4
        return (x * scale, (h - y) * scale)

    filled, holes = separate_regions(mask)
    if not filled:
        return (None, (0, 0))

    rects = mask.get_bounding_rects()
    size = (max(r.right for r in rects), max(r.top for r in rects))

    for region in filled:
        points = trace_outline(region)
        pen.moveTo(transform(*points[0]))
        prev, curr = points[0], None
        for p in points[1:]:
            if curr and not collinear(prev, curr, p):
                pen.lineTo(transform(*curr))
                prev = curr
            curr = p
        pen.closePath()

    for region in holes:
        points = list(reversed(trace_outline(region)))
        pen.moveTo(transform(*points[0]))
        prev, curr = points[0], None
        for p in points[1:]:
            if curr and not collinear(prev, curr, p):
                pen.lineTo(transform(*curr))
                prev = curr
            curr = p
        pen.closePath()

    return (pen.glyph(), size)


def trace_outline(mask: pygame.mask.Mask) -> list:
    w, h = mask.get_size()

    def get(x, y):
        return 0 <= x < w and 0 <= y < h and mask.get_at((x, y))

    start = None
    for y in range(h):
        for x in range(w):
            if mask.get_at((x, y)):
                start = (x, y)
                break
        if start:
            break

    result = [start]
    pos, facing = start, "right"

    while True:
        x, y = pos
        tl, tr = get(x - 1, y - 1), get(x, y - 1)
        bl, br = get(x - 1, y), get(x, y)

        if tl and br and not tr and not bl:
            pos = (x - 1, y) if facing == "up" else (x + 1, y)
            facing = "left" if facing == "up" else "right"
        elif tr and bl and not tl and not br:
            pos = (x, y - 1) if facing == "right" else (x, y + 1)
            facing = "up" if facing == "right" else "down"
        elif tl and not bl:
            pos, facing = (x - 1, y), "left"
        elif tr and not tl:
            pos, facing = (x, y - 1), "up"
        elif br and not tr:
            pos, facing = (x + 1, y), "right"
        elif bl and not br:
            pos, facing = (x, y + 1), "down"

        result.append(pos)
        if pos == start:
            break

    return result


def separate_regions(mask: pygame.mask.Mask) -> tuple:
    filled = mask.connected_components()
    w, h = mask.get_size()

    inv = pygame.mask.Mask((w + 2, h + 2))
    inv.draw(mask, (1, 1))
    inv.invert()

    holes = []
    checked = set()

    for y in range(h + 2):
        for x in range(w + 2):
            if (x, y) in checked or not inv.get_at((x, y)):
                continue

            region = pygame.mask.Mask((w + 2, h + 2))
            queue = [(x, y)]
            while queue:
                px, py = queue.pop()
                if (px, py) in checked or px < 0 or py < 0 or px >= w + 2 or py >= h + 2:
                    continue
                if not inv.get_at((px, py)):
                    continue
                checked.add((px, py))
                region.set_at((px, py), 1)
                queue.extend([(px - 1, py), (px + 1, py), (px, py - 1), (px, py + 1)])

            if region.get_at((0, 0)):
                continue

            fixed = pygame.mask.Mask((w, h))
            fixed.draw(region, (-1, -1))
            holes.append(fixed)

    return (filled, holes)


def collinear(p1: tuple, p2: tuple, p3: tuple) -> bool:
    return abs((p2[0] - p1[0]) * (p3[1] - p1[1]) - (p3[0] - p1[0]) * (p2[1] - p1[1])) < 1e-12


def main():
    parser = argparse.ArgumentParser(description="Generate Minecraft TTF fonts")
    parser.add_argument("--modern", action="store_true", help="Only modern fonts")
    parser.add_argument("--legacy", action="store_true", help="Only legacy fonts")
    parser.add_argument("--version", help="Minecraft version (default: latest)")
    args = parser.parse_args()

    do_modern = args.modern or not (args.modern or args.legacy)
    do_legacy = args.legacy or not (args.modern or args.legacy)

    aglfn = get_aglfn()

    if do_modern:
        version = args.version or get_latest_release()
        print(f"\n{'=' * 60}")
        print(f"Generating MODERN fonts (Minecraft {version})")
        print("=" * 60)
        generate_modern_fonts(version, aglfn)

    if do_legacy:
        print(f"\n{'=' * 60}")
        print(f"Generating LEGACY fonts (Minecraft {LEGACY_VERSION})")
        print("=" * 60)
        generate_legacy_fonts(aglfn)

    print("\nDone!")


if __name__ == "__main__":
    main()
