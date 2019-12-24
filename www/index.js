import { memory } from "okzoomer/okzoomer_bg";
import { Universe, set_width, set_height, get_cell } from "okzoomer";
import { hsl_to_rgb } from "./hsl_to_rgb";

console.time("JS initialization");

// Pull possible url params or set defaults
let params = new URL(document.location).searchParams;
let x = parseFloat(params.get("x") || -0.5);
let y = parseFloat(params.get("y") || -0.1);
let zoom = parseFloat(params.get("zoom") || 1.0);
console.log("Coordinates: X", x, " Y", y, " Zoom", zoom);
let invertControls = false;

// Info display
const info = document.createElement("p");
info.id = "info";
document.body.appendChild(info);
info.innerHTML = `x: ${x}, y: ${y}, zoom: ${zoom}`;

// Set up canvas to draw on + WebGL context for it
const canvas = document.createElement("canvas");
document.body.appendChild(canvas);
canvas.width = window.innerWidth;
canvas.height = window.innerHeight;

let width = canvas.width;
let height = canvas.height;
console.log("Width: ", width);
console.log("Height: ", height);
const universe = Universe.new(width, height);
const ctx = canvas.getContext("2d");
console.log(ctx);
const imageData = ctx.createImageData(canvas.width, canvas.height);

console.timeEnd("JS initialization");

const renderWithImageData = () => {
  console.time("JS renderWithImageData");
  const memoryPtr = universe.memoryptr();
  const importMemory = new Uint8Array(
    memory.buffer,
    memoryPtr,
    width * height * 4
  );

  const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
  for (let i = 0; i < imageData.data.length; i++) {
    imageData.data[i] = importMemory[i];
  }

  ctx.putImageData(imageData, 0, 0);
  console.timeEnd("JS renderWithImageData");
};

const redraw = () => {
  universe.draw(x, y, zoom);
  renderWithImageData();
  info.innerHTML = `x: ${x}, y: ${y}, zoom: ${zoom}`;
};

// start
redraw();

// Controls
document.onkeydown = e => {
  let xdiff = 0;
  let ydiff = 0;
  let zdiff = 0;
  switch (e.keyCode) {
    case 37:
      // left
      xdiff = (0.1 / zoom) * x;
      break;
    case 38:
      // up
      ydiff = (0.1 / zoom) * y;
      break;
    case 39:
      // right
      xdiff = -(0.1 / zoom) * x;
      break;
    case 40:
      // down
      ydiff = -(0.1 / zoom) * y;
      break;
    case 87:
      // w
      zdiff = 0.1 * zoom;
      break;
    case 83:
      // w
      zdiff = -0.1 * zoom;
      break;
  }
  if (invertControls) {
    xdiff *= -1;
    ydiff *= -1;
    zdiff *= -1;
  }
  // Only re-render if coords change
  x = x + xdiff;
  y = y + ydiff;
  zoom = zoom + zdiff;
  if ([xdiff, ydiff, zdiff].some(n => n !== 0)) {
    redraw();
  }
};
