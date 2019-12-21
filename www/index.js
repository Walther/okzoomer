import { memory } from "okzoomer/okzoomer_bg";
import { Universe, set_width, set_height, get_cell } from "okzoomer";
import { hsl_to_rgb } from "./hsl_to_rgb";

// Pull possible url params or set defaults
let params = new URL(document.location).searchParams;
let x = parseFloat(params.get("x") || -0.5);
let y = parseFloat(params.get("y") || 0.0);
let zoom = parseFloat(params.get("zoom") || 1.0);
console.log("Coordinates: X", x, " Y", y, " Zoom", zoom);

// Set up canvas to draw on + WebGL context for it
const canvas = document.createElement("canvas");
document.body.appendChild(canvas);
canvas.width = window.innerWidth;
canvas.height = window.innerHeight;

// TODO: this  allows for fun "big-pixel" display with the rect-based drawCells.
// Could be fun to extend  that support to the more efficient drawCellsData call,
// and parametrize this as "granularity" or something :think
let CELL_SIZE = 1; // px
let width = Math.floor(canvas.width / CELL_SIZE);
let height = Math.floor(canvas.height / CELL_SIZE);
console.log("Width: ", width);
console.log("Height: ", height);
const universe = Universe.new(width, height);
const ctx = canvas.getContext("2d");
console.log(ctx);
const imageData = ctx.createImageData(canvas.width, canvas.height);

const getIndex = (row, column) => {
  return row * width + column;
};

const renderWithRectangles = () => {
  const start = performance.now();
  const cellsPtr = universe.cellsptr();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

  ctx.beginPath();

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col);

      // let hue = universe.get_cell(row, col);
      const hue = cells[idx] + 30; // TODO: time-dependent?
      ctx.fillStyle = `hsl( ${hue}, 90%, 50% )`;

      ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
    }
  }
  const end = performance.now();
  const delta = end - start;
  console.log("JS renderWithRectangles() ", delta, "ms");

  ctx.stroke();
};

const renderWithImageData = () => {
  const start = performance.now();
  const cellsPtr = universe.cellsptr();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

  //  iterate over imagedata, rgba in linear memory
  const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
  var data = imageData.data;
  for (let i = 0; i < data.length; i += 4) {
    const value = cells[i / 4] + 30;
    const rgb = hsl_to_rgb(value, 0.9, 0.5);

    data[i] = 255 * rgb[0]; // r
    data[i + 1] = 255 * rgb[1]; // g
    data[i + 2] = 255 * rgb[2]; // b
    data[i + 3] = 255; // a
  }

  ctx.putImageData(imageData, 0, 0);
  const end = performance.now();
  const delta = end - start;
  console.log("JS renderWithImageData() ", delta, "ms");
};

// const renderLoop = () => {
//   universe.draw(x, y, zoom);
//   drawCells();

//   requestAnimationFrame(renderLoop);
// };

// start
universe.draw(x, y, zoom);
renderWithImageData();

// requestAnimationFrame(renderLoop);

// Helpful little function to enable realtime resize of the browser window
// (function() {
//   window.addEventListener("resize", resizeCanvas, false);
//   function resizeCanvas() {
//     canvas.width = window.innerWidth;
//     canvas.height = window.innerHeight;
//     width = canvas.width / (CELL_SIZE + 1) - 1;
//     height = canvas.height / (CELL_SIZE + 1) - 1;
//     universe.set_width(width);
//     universe.set_height(height);
//     drawStuff();
//   }
//   resizeCanvas();
//   function drawStuff() {
//
//     drawCells();
//     requestAnimationFrame(renderLoop);
//   }
// })();
