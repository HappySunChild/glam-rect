//! # glam_rect
//!
//! An extension for glam that adds support for rects, rectangles, rect regions, or whatever you prefer to call them and maybe more in the future.
//!
//! ## Features
//! * [Rect] (`f32`)
//! * [DRect] (`f64`)
//! * [PxRect]

pub mod f32;
pub use self::f32::Rect;

pub mod f64;
pub use self::f64::DRect;

pub mod px;
pub use self::px::PxRect;
