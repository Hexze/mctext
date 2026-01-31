const { writeFileSync } = require("fs");
const { createCanvas } = require("canvas");
const { MCText, FontSystem, LayoutOptions, render } = require("@hexze/mctext");

const fonts = FontSystem.modern();

const legacy = MCText.parse("§cr§6a§ei§an§bb§9o§dw §ftext");

const bold = new MCText()
  .span("bold ")
  .color("red")
  .bold()
  .then("& ")
  .color("gray")
  .then("italic")
  .color("aqua")
  .italic()
  .build();

const wrapped = new MCText()
  .span("this text wraps across multiple lines")
  .color("gold")
  .build();

const [width, height] = [300, 120];
const options = new LayoutOptions(24.0).withMaxWidth(300.0).withShadow(true);

const canvas = createCanvas(width, height);
const ctx = canvas.getContext("2d");
ctx.fillStyle = "rgb(24, 24, 24)";
ctx.fillRect(0, 0, width, height);

for (const [text, y] of [
  [legacy, 6],
  [bold, 36],
  [wrapped, 66],
]) {
  const result = render(fonts, text, width, height, options);
  const tmp = createCanvas(result.width(), result.height());
  const tmpCtx = tmp.getContext("2d");
  const imageData = tmpCtx.createImageData(result.width(), result.height());
  imageData.data.set(result.data());
  tmpCtx.putImageData(imageData, 0, 0);
  ctx.drawImage(tmp, 10, y);
}

writeFileSync("javascript_output2.png", canvas.toBuffer("image/png"));
