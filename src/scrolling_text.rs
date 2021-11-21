//! Support for scrolling ascii text horizontally.
//!
//! # Example
//!
//! ```ignore
//! use microbit::display::nonblocking::{Display, Frame, MicrobitFrame};
//! use microbit_text::scrolling::Animate;
//! use microbit_text::scrolling_text::ScrollingStaticText;
//! let mut display = MicrobitDisplay::new(...);
//! let mut scroller = ScrollingStaticText::default();
//! let frame = MicrobitFrame::default();
//! scroller.set_message(b"Hello, world!");
//! while !scroller.is_finished() {
//!     // every 50ms or so
//!     scroller.tick();
//!     frame.set(scroller);
//!     display.show_frame(frame);
//! }
//! ```
//!

use tiny_led_matrix::Render;

use crate::image::BitImage;
use crate::font;
use crate::scrolling::{Animate, ScrollingState, Scrollable};

/// A [`Scrollable`] displaying a static ascii byte-string slice.
#[derive(Default)]
#[derive(Copy, Clone)]
pub struct ScrollingStaticText {
    message: &'static [u8],
    state: ScrollingState,
}

impl ScrollingStaticText {

    /// Specifies the ascii byte-string slice to be displayed.
    ///
    /// This also resets the animation to the beginning.
    pub fn set_message(&mut self, message: &'static [u8]) {
        self.message = message;
        self.reset();
    }

}

impl Scrollable for ScrollingStaticText {

    type Subimage = BitImage;

    fn length(&self) -> usize {
        self.message.len()
    }

    fn state(&self) -> &ScrollingState {
        &self.state
    }

    fn state_mut(&mut self) -> &mut ScrollingState {
        &mut self.state
    }

    fn subimage(&self, index: usize) -> &BitImage {
        font::character(self.message[index])
    }

}


impl Render for ScrollingStaticText {

    fn brightness_at(&self, x: usize, y: usize) -> u8 {
        self.current_brightness_at(x, y)
    }

}


/// A [`Scrollable`] displaying an ascii byte-string of up to `N` bytes.
#[derive(Copy, Clone)]
pub struct ScrollingBufferedText<const N: usize> {
    length: usize,
    message: [u8; N],
    state: ScrollingState,
}

impl<const N: usize> ScrollingBufferedText<N> {

    /// Specifies the ascii byte-string to be displayed.
    ///
    /// Makes a copy of the byte-string.
    ///
    /// This also resets the animation to the beginning.
    ///
    /// # Panics
    ///
    /// Panics if `message` is more than `N` bytes long.
    pub fn set_message(&mut self, message: &[u8]) {
        assert!(message.len() <= N, "message too long");
        self.length = message.len();
        self.message[..self.length].copy_from_slice(message);
        self.reset();
    }


}

impl<const N: usize> Default for ScrollingBufferedText<N> {

    fn default() -> ScrollingBufferedText<N> {
        ScrollingBufferedText {
            length: 0,
            message: [0; N],
            state: Default::default(),
        }
    }

}

impl<const N: usize> Scrollable for ScrollingBufferedText<N> {

    type Subimage = BitImage;

    fn length(&self) -> usize {
        self.length
    }

    fn state(&self) -> &ScrollingState {
        &self.state
    }

    fn state_mut(&mut self) -> &mut ScrollingState {
        &mut self.state
    }

    fn subimage(&self, index: usize) -> &BitImage {
        font::character(self.message[index])
    }

}

impl<const N: usize> Render for ScrollingBufferedText<N> {

    fn brightness_at(&self, x: usize, y: usize) -> u8 {
        self.current_brightness_at(x, y)
    }

}

