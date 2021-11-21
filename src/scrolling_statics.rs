//! Support for scrolling arbitrary static images horizontally.
//!
//! [`ScrollingStatics`] can be used for static slices of any type implementing
//! [`Render`].
//!
//! # Example
//!
//! ```ignore
//! use microbit::display::nonblocking::{Display, Frame, MicrobitFrame};
//! use microbit_text::scrolling::Animate;
//! use microbit_text::scrolling_text::ScrollingStaticText;
//! use microbit_text::image::BitImage;
//! const BLANK: BitImage = BitImage::blank();
//! const HEART: BitImage = BitImage::new(&[
//!     [0, 1, 0, 1, 0],
//!     [1, 0, 1, 0, 1],
//!     [1, 0, 0, 0, 1],
//!     [0, 1, 0, 1, 0],
//!     [0, 0, 1, 0, 0],
//! ]);
//! let mut display = MicrobitDisplay::new(...);
//! let mut scroller = ScrollingStatics::default();
//! let frame = MicrobitFrame::default();
//! scroller.set_images(&[&HEART, &BLANK, &HEART]);
//! while !scroller.is_finished() {
//!     // every 50ms or so
//!     scroller.tick();
//!     frame.set(scroller);
//!     display.show_frame(frame);
//! }
//! ```
//!
//! [`Render`]: tiny_led_matrix::Render

use tiny_led_matrix::Render;

use crate::scrolling::{Animate, ScrollingState, Scrollable};

/// A [`Scrollable`] displaying a static slice of arbitrary images.
///
/// The underlying images can be any sized type that implements [`Render`].
#[derive(Copy, Clone)]
pub struct ScrollingStatics<T: Render + 'static> {
    images: &'static [T],
    state: ScrollingState,
}

impl<T: Render + 'static> ScrollingStatics<T> {

    /// Specifies the images to be displayed.
    ///
    /// This also resets the animation to the beginning.
    pub fn set_images(&mut self, images: &'static [T]) {
        self.images = images;
        self.reset();
    }


}

impl<T: Render + 'static> Default for ScrollingStatics<T> {

    fn default() -> ScrollingStatics<T> {
        ScrollingStatics {
            images: &[],
            state: Default::default(),
        }
    }

}

impl<T: Render + 'static> Scrollable for ScrollingStatics<T> {

    type Subimage = T;

    fn length(&self) -> usize {
        self.images.len()
    }

    fn state(&self) -> &ScrollingState {
        &self.state
    }

    fn state_mut(&mut self) -> &mut ScrollingState {
        &mut self.state
    }

    fn subimage(&self, index: usize) -> &T {
        &self.images[index]
    }

}

impl<T: Render + 'static> Render for ScrollingStatics<T> {

    fn brightness_at(&self, x: usize, y: usize) -> u8 {
        self.current_brightness_at(x, y)
    }

}

