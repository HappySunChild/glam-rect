use glam::Vec2;
use std::fmt;

#[derive(Clone, Copy, PartialEq)]
pub struct Rect {
	pub x: f32,
	pub y: f32,
	pub w: f32,
	pub h: f32,
}

impl Rect {
	/// Creates a new [Rect] with its top-left corner positioned at (`x`, `y`), with size (`w`, `h`).
	#[inline(always)]
	pub const fn from_xywh(x: f32, y: f32, w: f32, h: f32) -> Self {
		Self { x, y, w, h }
	}

	/// Functionally similar to [Rect::from_xywh], but with [Vec2]'s instead of individual [f32]s.
	///
	/// Creates a new [Rect] with its top-left corner positioned at (`pos.x`, `pos.y`), with size (`size.x`, `size.y`).
	#[inline(always)]
	pub const fn from_vec2_xywh(pos: Vec2, size: Vec2) -> Self {
		Self {
			x: pos.x,
			y: pos.y,
			w: size.x,
			h: size.y,
		}
	}

	/// Creates a new [Rect] with its top-left corner positioned at (`left`, `top`)
	/// and its bottom-right corner positioned at (`right`, `bottom`).
	#[inline(always)]
	pub const fn from_ltrb(left: f32, top: f32, right: f32, bottom: f32) -> Self {
		Self {
			x: left,
			y: top,
			w: right - left,
			h: bottom - top,
		}
	}

	/// Functionally similar to [Rect::from_ltrb], but with [Vec2]'s instead of individual [f32]s.
	///
	/// Creates a new [Rect] with its top-left corner positioned at (`lt.x`, `lt.y`)
	/// and its bottom-right corner positioned at (`rb.x`, `rb.y`).
	#[inline(always)]
	pub const fn from_vec2_ltrb(lt: Vec2, rb: Vec2) -> Self {
		Self::from_ltrb(lt.x, lt.y, rb.x, rb.y)
	}

	/// Returns the size of the [Rect] as a [Vec2] with components (`w`, `h`).
	#[inline(always)]
	pub fn size(&self) -> Vec2 {
		Vec2::new(self.w, self.h)
	}

	/// Returns the y-coordinate of the top edge of the [Rect].
	///
	/// This is typically equivalent to [Rect::y].
	#[inline(always)]
	pub fn top(&self) -> f32 {
		self.y
	}

	/// Returns the x-coordinate of the left edge of the [Rect].
	///
	/// This is typically equivalent to [Rect::x].
	#[inline(always)]
	pub fn left(&self) -> f32 {
		self.x
	}

	/// Returns the y-coordinate of the bottom edge of the [Rect].
	#[inline(always)]
	pub fn bottom(&self) -> f32 {
		self.y + self.h
	}

	/// Returns the x-coordinate of the right edge of the [Rect].
	#[inline(always)]
	pub fn right(&self) -> f32 {
		self.x + self.w
	}

	/// Returns the position of the top-left corner as a [Vec2].
	#[inline(always)]
	pub fn top_left(&self) -> Vec2 {
		Vec2::new(self.x, self.y)
	}

	/// Returns the position of the top-right corner as a [Vec2].
	#[inline(always)]
	pub fn top_right(&self) -> Vec2 {
		Vec2::new(self.x + self.w, self.y)
	}

	/// Returns the position of the bottom-left corner as a [Vec2].
	#[inline(always)]
	pub fn bottom_left(&self) -> Vec2 {
		Vec2::new(self.x, self.y + self.h)
	}

	/// Returns the position of the bottom-right corner as a [Vec2].
	#[inline(always)]
	pub fn bottom_right(&self) -> Vec2 {
		Vec2::new(self.x + self.w, self.y + self.h)
	}

	/// Returns the position of the center of the [Rect] as a [Vec2].
	#[inline(always)]
	pub fn center(&self) -> Vec2 {
		Vec2::new(self.x + self.w / 2.0, self.y + self.h / 2.0)
	}

	/// Returns whether the point at (`x`, `y`) is contained within the [Rect].
	#[inline(always)]
	pub fn contains_point(&self, x: f32, y: f32) -> bool {
		x >= self.x && x <= self.right() && y >= self.y && y <= self.bottom()
	}

	/// Functionally similar to [Rect::contains_point], but with [Vec2]'s instead of individual [f32]s.
	///
	/// Returns whether the point at (`point.x`, `point.y`) is contained within the [Rect].
	#[inline(always)]
	pub fn contains_vec2_point(&self, point: Vec2) -> bool {
		self.contains_point(point.x, point.y)
	}

	/// Returns whether the given [Rect] fits within the [Rect].
	#[inline(always)]
	pub fn contains_rect(&self, other: Rect) -> bool {
		self.contains_point(other.x, other.y) && self.contains_point(other.right(), other.bottom())
	}

	/// Shifts over the [Rect] by (`x_offset`, `y_offset`).
	#[inline(always)]
	pub fn shift_over(mut self, x_offset: f32, y_offset: f32) -> Self {
		self.x += x_offset;
		self.y += y_offset;
		self
	}

	/// Functionally similar to [Rect::shift_over], but with [Vec2]'s instead of individual [f32]s.
	///
	/// Shifts over the [Rect] by (`offset.x`, `offset.y`).
	#[inline(always)]
	pub fn shift_over_vec2(self, offset: Vec2) -> Self {
		self.shift_over(offset.x, offset.y)
	}

	/// Resizes the [Rect] by (`w_offset`, `h_offset`).
	#[inline(always)]
	pub fn resize_by(mut self, w_offset: f32, h_offset: f32) -> Self {
		self.w += w_offset;
		self.h += h_offset;
		self
	}

	/// Functionally similar to [Rect::resize_by], but with [Vec2]'s instead of individual [f32]s.
	///
	/// Resizes the [Rect] by (`offset.x`, `offset.y`).
	#[inline(always)]
	pub fn resize_by_vec2(self, offset: Vec2) -> Self {
		self.resize_by(offset.x, offset.y)
	}
}

impl fmt::Display for Rect {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if let Some(p) = f.precision() {
			write!(
				f,
				"[{:.*}, {:.*}, {:.*}, {:.*}]",
				p, self.x, p, self.y, p, self.w, p, self.h
			)
		} else {
			write!(f, "[{}, {}, {}, {}]", self.x, self.y, self.w, self.h)
		}
	}
}

impl fmt::Debug for Rect {
	fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt.debug_tuple(stringify!(Rect))
			.field(&self.x)
			.field(&self.y)
			.field(&self.w)
			.field(&self.h)
			.finish()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn constructs_xywh() {
		let new_rect = Rect::from_xywh(0.0, 0.0, 0.0, 0.0);
		assert_eq!(new_rect.x, 0.0);
		assert_eq!(new_rect.y, 0.0);
		assert_eq!(new_rect.w, 0.0);
		assert_eq!(new_rect.h, 0.0);

		let new_rect = Rect::from_xywh(-10.0, -293.39, 0.75, 10.3);
		assert_eq!(new_rect.x, -10.0);
		assert_eq!(new_rect.y, -293.39);
		assert_eq!(new_rect.w, 0.75);
		assert_eq!(new_rect.h, 10.3);
	}

	#[test]
	fn constructs_vec2_xywh() {
		let new_rect = Rect::from_vec2_xywh(Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0));
		assert_eq!(new_rect.x, 0.0);
		assert_eq!(new_rect.y, 0.0);
		assert_eq!(new_rect.w, 0.0);
		assert_eq!(new_rect.h, 0.0);

		let new_rect = Rect::from_vec2_xywh(Vec2::new(-10.0, -293.39), Vec2::new(0.75, 10.3));
		assert_eq!(new_rect.x, -10.0);
		assert_eq!(new_rect.y, -293.39);
		assert_eq!(new_rect.w, 0.75);
		assert_eq!(new_rect.h, 10.3);
	}

	#[test]
	fn constructs_ltrb() {
		let new_rect = Rect::from_ltrb(0.0, 0.0, 0.0, 0.0);
		assert_eq!(new_rect.x, 0.0);
		assert_eq!(new_rect.y, 0.0);
		assert_eq!(new_rect.w, 0.0);
		assert_eq!(new_rect.h, 0.0);

		let new_rect = Rect::from_ltrb(0.0, 0.0, 100.0, 100.0);
		assert_eq!(new_rect.x, 0.0);
		assert_eq!(new_rect.y, 0.0);
		assert_eq!(new_rect.w, 100.0);
		assert_eq!(new_rect.h, 100.0);

		let new_rect = Rect::from_ltrb(50.0, 50.0, 100.0, 100.0);
		assert_eq!(new_rect.x, 50.0);
		assert_eq!(new_rect.y, 50.0);
		assert_eq!(new_rect.w, 50.0);
		assert_eq!(new_rect.h, 50.0);
	}

	#[test]
	fn construct_vec2_ltrb() {
		let new_rect = Rect::from_vec2_ltrb(Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0));
		assert_eq!(new_rect.x, 0.0);
		assert_eq!(new_rect.y, 0.0);
		assert_eq!(new_rect.w, 0.0);
		assert_eq!(new_rect.h, 0.0);

		let new_rect = Rect::from_vec2_ltrb(Vec2::new(0.0, 0.0), Vec2::new(100.0, 100.0));
		assert_eq!(new_rect.x, 0.0);
		assert_eq!(new_rect.y, 0.0);
		assert_eq!(new_rect.w, 100.0);
		assert_eq!(new_rect.h, 100.0);

		let new_rect = Rect::from_vec2_ltrb(Vec2::new(50.0, 50.0), Vec2::new(100.0, 100.0));
		assert_eq!(new_rect.x, 50.0);
		assert_eq!(new_rect.y, 50.0);
		assert_eq!(new_rect.w, 50.0);
		assert_eq!(new_rect.h, 50.0);
	}
}
