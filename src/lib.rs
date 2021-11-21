//! Scrolling text on a 5×5 display (for example, a
//! [micro:bit](https://microbit.org/)).
//!
//! # Features
//!
//! This crate provides:
//! - a simple 5×5 image type;
//! - a copy of the 'pendolino' font from the [micro:bit runtime][dal];
//! - support for scrolling a sequence of 5×5 images;
//! - support for scrolling text.
//!
//! These are all for use with the [`tiny-led-matrix`] crate.
//!
//! # Simple images
//!
//! The [`image`] module provides a [`BitImage`] type for non-greyscale 5x5
//! images.
//!
//! # Fonts
//!
//! The [`font`] module provides 5×5 representations of the ascii printable
//! characters as [`BitImage`]s.
//!
//! These are taken from the "pendolino" font supplied with the
//! [micro:bit runtime][dal].
//!
//! # Scrolling images and text
//!
//! The [`scrolling_text`] module supports horizontally scrolling messages,
//! providing [`ScrollingStaticText`] and [`ScrollingBufferedText`] types.
//!
//! The [`scrolling_statics`] module supports horizontal scrolling for an
//! arbitrary static sequence of images via a [`ScrollingStatics`] type.
//!
//! The [`scrolling`] module provides interfaces used by types implementing
//! horizontal scrolling, including the [`Animate`] trait used to control the
//! scrolling behaviour.
//!
//! [dal]: https://lancaster-university.github.io/microbit-docs/
//! [`tiny-led-matrix`]: https://docs.rs/tiny-led-matrix/
//! [`Render`]: tiny_led_matrix::Render
//! [`Animate`]: scrolling::Animate
//! [`BitImage`]: image::BitImage
//! [`Scrollable`]: scrolling::Scrollable
//! [`ScrollingStatics`]: scrolling_statics::ScrollingStatics
//! [`ScrollingBufferedText`]: scrolling_text::ScrollingBufferedText
//! [`ScrollingStaticText`]: scrolling_text::ScrollingStaticText

#![no_std]

pub mod image;
pub mod font;
pub mod scrolling;
pub mod scrolling_statics;
pub mod scrolling_text;
