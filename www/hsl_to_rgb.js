// HSL to RGB alternative
// adapted from wikipedia 2019-12-21 by Walther
// https://en.wikipedia.org/wiki/HSL_and_HSV#HSL_to_RGB_alternative

// Internal helper functions
const a = (s, l) => s * Math.min(l, 1 - l);
const k = (h, n) => (n + h / 30) % 12;
const n = (h, s, l, n) =>
  l - a(s, l) * Math.max(Math.min(k(h, n) - 3, 9 - k(h, n), 1), -1);

/**
 *
 * @param {Number} h Hue, integer degrees  0-360
 * @param {Number} s Saturation, float 0-1
 * @param {Number} l Lightness, float 0-1
 *
 * @returns {[Number]} [r,g,b], 0-1 each
 */
export const hsl_to_rgb = (h, s, l) => [
  n(h, s, l, 0),
  n(h, s, l, 8),
  n(h, s, l, 4)
];
