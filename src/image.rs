//! Static 5×5 black-and-white images.

use tiny_led_matrix::{Render, MAX_BRIGHTNESS};

/// A 5×5 image supporting only two levels of brightness (on and off).
///
/// Uses 5 bytes of storage.
///
/// For display, each pixel is treated as having brightness either 0 or
/// [MAX_BRIGHTNESS].
///
/// [MAX_BRIGHTNESS]: tiny_led_matrix::MAX_BRIGHTNESS

#[derive(Copy, Clone, Debug)]
pub struct BitImage (
    [u8; 5]
);

impl BitImage {

    /// Constructs a BitImage from an array of brightnesses.
    ///
    /// The data should be an array of 5 rows (top first), each of which is an
    /// array of 5 values (left first). Each value should be either 0 or 1.
    ///
    /// # Example
    ///
    /// ```
    /// const HEART: BitImage = BitImage::new(&[
    ///     [0, 1, 0, 1, 0],
    ///     [1, 0, 1, 0, 1],
    ///     [1, 0, 0, 0, 1],
    ///     [0, 1, 0, 1, 0],
    ///     [0, 0, 1, 0, 0],
    /// ]);
    /// ```
    pub const fn new(im: &[[u8; 5]; 5]) -> BitImage {
        const fn row_byte(row: [u8; 5]) -> u8 {
            row[0] | row[1]<<1 | row[2]<<2 | row[3]<<3 | row[4]<<4
        }
        BitImage([
            row_byte(im[0]),
            row_byte(im[1]),
            row_byte(im[2]),
            row_byte(im[3]),
            row_byte(im[4]),
        ])
    }

    /// Returns a new blank BitImage.
    ///
    /// All pixel values are 0.
    pub const fn blank() -> BitImage {
        BitImage([0; 5])
    }

}

impl Render for BitImage {
    fn brightness_at(&self, x: usize, y: usize) -> u8 {
        let rowdata = self.0[y];
        if rowdata & (1<<x) != 0 {MAX_BRIGHTNESS as u8} else {0}
    }
}

impl Render for &BitImage {
    fn brightness_at(&self, x: usize, y: usize) -> u8 {
        BitImage::brightness_at(self, x, y)
    }
}

