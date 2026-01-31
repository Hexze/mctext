import mctext
from PIL import Image

fonts = mctext.FontSystem.modern()

legacy = mctext.MCText.parse("§cr§6a§ei§an§bb§9o§dw §ftext")

bold = (
    mctext.MCText()
    .span("bold ")
    .color("red")
    .bold()
    .then("& ")
    .color("gray")
    .then("italic")
    .color("aqua")
    .italic()
    .build()
)

wrapped = (
    mctext.MCText().span("this text wraps across multiple lines").color("gold").build()
)

width, height = 300, 120
options = mctext.LayoutOptions(24.0).with_max_width(300.0).with_shadow(True)

canvas = Image.new("RGBA", (width, height), (24, 24, 24, 255))
for text, y in [(legacy, 6), (bold, 36), (wrapped, 66)]:
    result = mctext.render(fonts, text, width, height, options)
    img = Image.frombytes("RGBA", (result.width, result.height), bytes(result.data()))
    canvas.alpha_composite(img, (10, y))

canvas.save("python_output2.png")
