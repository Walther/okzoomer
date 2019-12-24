pub fn set_panic_hook() {
  // When the `console_error_panic_hook` feature is enabled, we can call the
  // `set_panic_hook` function at least once during initialization, and then
  // we will get better error messages if our code ever panics.
  //
  // For more details see
  // https://github.com/rustwasm/console_error_panic_hook#readme
  #[cfg(feature = "console_error_panic_hook")]
  console_error_panic_hook::set_once();
}

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

/// HSL to RGB alternative
/// adapted from wikipedia 2019-12-24
/// https://en.wikipedia.org/wiki/HSL_and_HSV#HSL_to_RGB_alternative
///
/// h: 0-360, s: 0-1, l: 0-1
pub fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (f32, f32, f32) {
  // Internal helper functions

  fn min3(a: f32, b: f32, c: f32) -> f32 {
    a.min(b.min(c))
  }

  fn a(s: f32, l: f32) -> f32 {
    s * l.min(1.0 - l)
  }
  fn k(h: f32, n: f32) -> f32 {
    (n + h / 30.0) % 12.0
  }
  fn n(h: f32, s: f32, l: f32, n: f32) -> f32 {
    l - a(s, l) * min3(k(h, n) - 3.0, 9.0 - k(h, n), 1.0).max(-1.0)
  }

  (n(h, s, l, 0.0), n(h, s, l, 8.0), n(h, s, l, 4.0))
}
