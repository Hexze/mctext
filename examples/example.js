const { writeFileSync } = require("fs");
const { createCanvas } = require("canvas");
const { MCText, FontSystem, LayoutOptions, render } = require("@hexze/mctext");

const fonts = FontSystem.modern();

const text = new MCText()
  .span("hello ")
  .color("red")
  .then("world!")
  .color("gold")
  .build();

const [width, height] = [200, 60];
const options = new LayoutOptions(32.0).withShadow(true);

const result = render(fonts, text, width, height, options);

const canvas = createCanvas(width, height);
const ctx = canvas.getContext("2d");
ctx.fillStyle = "rgb(24, 24, 24)";
ctx.fillRect(0, 0, width, height);

const tmp = createCanvas(result.width(), result.height());
const tmpCtx = tmp.getContext("2d");
const imageData = tmpCtx.createImageData(result.width(), result.height());
imageData.data.set(result.data());
tmpCtx.putImageData(imageData, 0, 0);
ctx.drawImage(tmp, 10, 14);

writeFileSync("javascript_output.png", canvas.toBuffer("image/png"));
