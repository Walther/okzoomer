mod utils;
use utils::{hsl_to_rgb, Timer};

use num_complex::Complex;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
/// The main structure for this fractal zoomer
pub struct Universe {
    /// Width of the drawing area in pixels
    width: u32,
    /// Height of the drawing area in pixels
    height: u32,
    /// Array of RBGA; 4 bytes per pixel
    memory: Vec<u8>,
}

const MAX_ITERATIONS: u8 = 255;

#[wasm_bindgen]
impl Universe {
    pub fn draw(&mut self, location_x: f32, location_y: f32, zoom: f32) {
        let _timer = Timer::new("WASM Universe::draw");

        // zoom-in size; which coords are we looking at
        // TODO: think about normalizing aspect ratio for viewport
        let cxmin = location_x - 2.0 / zoom;
        let cxmax = location_x + 2.0 / zoom;
        let cymin = location_y - 1.5 / zoom;
        let cymax = location_y + 1.5 / zoom;

        let scalex = (cxmax - cxmin) / self.width as f32;
        let scaley = (cymax - cymin) / self.height as f32;

        let mut memory_index = 0;
        while memory_index < self.memory.len() {
            let pixel_index = memory_index / 4;
            let x = pixel_index_to_x(&pixel_index, self.width);
            let y = pixel_index_to_y(&pixel_index, self.width);

            let cx = cxmin + x as f32 * scalex;
            let cy = cymin + y as f32 * scaley;

            let c = Complex::new(cx, cy);
            let mut z = Complex::new(0f32, 0f32);

            let mut iteration = 0;
            for test in 0..MAX_ITERATIONS {
                // if z.norm() > 2.0 {
                // (norm squared) > (2 squared): faster than the norm > 2 above
                if z.norm_sqr() > 4.0 {
                    // bail out
                    break;
                }
                z = z * z + c;
                iteration = test;
            }

            // use iteration count for colorization, as the hue in hsl space
            let hue = iteration as f32 + 30.0;
            let rgb = hsl_to_rgb(hue, 1.0, 0.5);

            self.memory[memory_index] = (255.0 * rgb.0) as u8; // r
            self.memory[memory_index + 1] = (255.0 * rgb.1) as u8; // g
            self.memory[memory_index + 2] = (255.0 * rgb.2) as u8; // b
            self.memory[memory_index + 3] = 255; // a
            memory_index += 4;
        }
    }

    pub fn new(width: u32, height: u32) -> Universe {
        let _timer = Timer::new("WASM Universe::new");
        utils::set_panic_hook();
        // is there a prettier way to initialize a vec :|
        let memory = (0..width * height * 4).map(|i| 0).collect();

        Universe {
            width,
            height,
            memory,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn memoryptr(&self) -> *const u8 {
        let _timer = Timer::new("WASM Universe::memoryptr");
        self.memory.as_ptr()
    }
}

// ugly helpers  to avoid ownership issues
fn pixel_index_to_y(idx: &usize, width: u32) -> usize {
    idx / width as usize // flooring division
}
fn pixel_index_to_x(idx: &usize, width: u32) -> usize {
    idx % width as usize // modulo
}
