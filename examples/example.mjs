import { writeFileSync } from "fs";
import { createCanvas } from "canvas";
import {
  MCText,
  FontSystem,
  LayoutOptions,
  render,
} from "../bindings/wasm/pkg/mctext_wasm.js";

class TextRenderer {
  constructor() {
    this.fonts = FontSystem.modern();
  }

  render(text, size = 16.0) {
    let width = Math.ceil(this.fonts.measure(text.plainText(), size)) + 8;
    let height = Math.ceil(size * 2);
    let options = new LayoutOptions(size).withShadow(true);
    let result = render(this.fonts, text, width, height, options);

    let tmpCanvas = createCanvas(result.width(), result.height());
    let tmpCtx = tmpCanvas.getContext("2d");
    let imageData = tmpCtx.createImageData(result.width(), result.height());
    imageData.data.set(result.data());
    tmpCtx.putImageData(imageData, 0, 0);

    return tmpCanvas;
  }
}

let renderer = new TextRenderer();

let canvas = createCanvas(400, 60);
let ctx = canvas.getContext("2d");
ctx.fillStyle = "black";
ctx.fillRect(0, 0, 400, 60);

let text = new MCText().span("Minecraft Text!").color("red").build();
ctx.drawImage(renderer.render(text), 10, 20);

writeFileSync("javascript_output.png", canvas.toBuffer("image/png"));
