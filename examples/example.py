from PIL import Image

import mctext


class TextRenderer:
    def __init__(self):
        self.fonts = mctext.FontSystem.modern()

    def render(self, text: mctext.MCText, size: float = 16.0):
        width = int(self.fonts.measure(text.plain_text(), size)) + 8
        height = int(size * 2)
        options = mctext.LayoutOptions(size, None, True)
        result = mctext.render(self.fonts, text, width, height, options)
        return Image.frombytes(
            "RGBA", (result.width, result.height), bytes(result.data())
        )


renderer = TextRenderer()

canvas = Image.new("RGBA", (400, 60), (0, 0, 0, 255))

text = mctext.MCText().add("Minecraft Text!").color("red").build()
canvas.alpha_composite(renderer.render(text), (10, 20))

canvas.save("python_output.png")
