mod utils;
use num_complex::Complex;
use wasm_bindgen::prelude::*;
use web_sys::console;

pub struct Timer<'a> {
    name: &'a str,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        console::time_with_label(name);
        Timer { name }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        console::time_end_with_label(self.name);
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<u8>,
}

const MAX_ITERATIONS: u8 = 255;

#[wasm_bindgen]
impl Universe {
    // Helper method for accessing from the vec
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    pub fn draw(&mut self, location_x: f32, location_y: f32, zoom: f32) {
        let _timer = Timer::new("Universe::draw");
        // viewport size
        let width = self.width;
        let height = self.height;

        // zoom-in size; which coords are we looking at
        // TODO: think about normalizing aspect ratio for viewport
        let cxmin = location_x - 2.0 / zoom;
        let cxmax = location_x + 2.0 / zoom;
        let cymin = location_y - 1.5 / zoom;
        let cymax = location_y + 1.5 / zoom;

        let scalex = (cxmax - cxmin) / width as f32;
        let scaley = (cymax - cymin) / height as f32;

        // TODO: rayon
        (0..width).for_each(|x| {
            (0..height).for_each(|y| {
                // pixel index in linear memory layout
                let idx = self.get_index(y, x);

                let cx = cxmin + x as f32 * scalex;
                let cy = cymin + y as f32 * scaley;

                let c = Complex::new(cx, cy);
                let mut z = Complex::new(0f32, 0f32);

                let mut iteration = 0;
                for test in 0..MAX_ITERATIONS {
                    if z.norm() > 2.0 {
                        // bail out
                        break;
                    }
                    z = z * z + c;
                    iteration = test;
                }

                self.cells[idx] = iteration
            })
        });
    }

    pub fn new(width: u32, height: u32) -> Universe {
        let _timer = Timer::new("Universe::new");
        utils::set_panic_hook();
        let cells = (0..width * height).map(|i| 0).collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cellsptr(&self) -> *const u8 {
        let _timer = Timer::new("Universe::cellsptr");
        self.cells.as_ptr()
    }

    pub fn get_cell(&self, row: u32, column: u32) -> u8 {
        let _timer = Timer::new("Universe::get_cell");
        let idx = self.get_index(row, column);
        self.cells[idx]
    }
}
