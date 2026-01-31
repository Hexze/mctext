import mctext
from PIL import Image

fonts = mctext.FontSystem.modern()

text = mctext.MCText().span("hello ").color("red").then("world!").color("gold").build()

width, height = 200, 60
options = mctext.LayoutOptions(32.0).with_shadow(True)

result = mctext.render(fonts, text, width, height, options)
img = Image.frombytes("RGBA", (result.width, result.height), bytes(result.data()))

canvas = Image.new("RGBA", (width, height), (24, 24, 24, 255))
canvas.alpha_composite(img, (10, 14))
canvas.save("python_output.png")
