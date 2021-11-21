//! Support for scrolling sequences of 5×5 images horizontally.
//!
//! Each kind of scrolling sequence is represented by a type implementing
//! [`Animate`] (for controlling the sequence) and [`Render`] (for displaying
//! it).
//!
//! To create a new kind of scrolling sequence, make a new implementation of
//! [`Scrollable`]. It can implement `Render::brightness_at` by delegating to
//! `self.current_brightness_at()`. [`Animate`] will be implemented
//! automatically.
//!
//! See [`scrolling_text`] for scrolling text strings.
//!
//! See [`scrolling_statics`] for scrolling arbitrary images.
//!
//! [`scrolling_text`]: crate::scrolling_text
//! [`scrolling_statics`]: crate::scrolling_statics

use tiny_led_matrix::Render;


/// The state of an animation.
pub trait Animate {

    /// Say whether the animation has completed.
    fn is_finished(&self) -> bool;

    /// Reset the  animation to the beginning.
    fn reset(&mut self);

    /// Advance to the next step of the animation.
    ///
    /// If the animation has completed, does nothing.
    fn tick(&mut self);

}


/// Data needed to record the state of a scrolling animation.
///
/// Implementations of [`Scrollable`] should contain one of these and make it
/// available via `state()` and `state_mut()`.
#[derive(Default)]
#[derive(Copy, Clone, Debug)]
pub struct ScrollingState {
    // index of the character being scrolled on, or about to be scrolled on
    index: usize,
    // 0..5
    pixel: usize,
}

impl ScrollingState {

    /// Reset the state to the beginning.
    pub fn reset(&mut self) {
        self.index = 0;
        self.pixel = 0;
    }

    /// Advance the state by one tick.
    pub fn tick(&mut self) {
        self.pixel += 1;
        if self.pixel == 5 {
            self.pixel = 0;
            self.index += 1;
        }
    }

}


/// A horizontally scrolling sequence of 5×5 images.
///
/// `Scrollable`s automatically implement [`Animate`].
///
/// When a `Scrollable` also implements `Render`, the rendered image is the
/// current state of the animation.
pub trait Scrollable {

    /// The type of the underlying 5×5 images.
    type Subimage: Render;

    /// The number of underlying images.
    fn length(&self) -> usize;

    /// A [`ScrollingState`] indicating the current point in the animation.
    fn state(&self) -> &ScrollingState;

    /// A [`ScrollingState`] indicating the current point in the animation, as
    /// a mutable reference.
    fn state_mut(&mut self) -> &mut ScrollingState;

    /// A reference to the underlying image at the specified index.
    fn subimage(&self, index: usize) -> &Self::Subimage;

    /// Returns the brightness value for a single LED in the current state.
    ///
    /// Use this to implement `Render`.
    fn current_brightness_at(&self, x: usize, y: usize) -> u8 {
        if self.state().index > self.length() {return 0}
        let state = self.state();
        let (index, x) = if x + state.pixel < 5 {
            if state.index == 0 {return 0}
            (state.index - 1, x + state.pixel)
        } else {
            if state.index == self.length() {return 0}
            (state.index, x + state.pixel - 5)
        };
        self.subimage(index).brightness_at(x, y)
    }
}


impl<T : Scrollable> Animate for T {

    fn is_finished(&self) -> bool {
        self.state().index > self.length()
    }

    fn reset(&mut self) {
        self.state_mut().reset();
    }

    fn tick(&mut self) {
        if !self.is_finished() {
            self.state_mut().tick();
        }
    }
}

