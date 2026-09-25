use glam::Vec2;
use std::fmt;

/// A 2-dimensional rectangle.
///
/// The `width` and `height` of a rectangle are technically allowed to be negative, but it is heavily discouraged.
///
/// # Examples
/// ```
/// use glam_rect::Rect;
///
/// let new_rect = Rect::from_xywh(0.0, 0.0, 10.0, 10.0);
/// assert!(new_rect.contains_point(1.0, 1.0));
/// ```
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
	///
	/// This is typically equivalent to the sum of [Rect::y] and [Rect::h].
	#[inline(always)]
	pub fn bottom(&self) -> f32 {
		self.y + self.h
	}

	/// Returns the x-coordinate of the right edge of the [Rect].
	///
	/// This is typically equivalent to the sum of [Rect::x] and [Rect::w].
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

	/// Returns the size of the [Rect] as a [Vec2] with components (`w`, `h`).
	#[inline(always)]
	pub fn size(&self) -> Vec2 {
		Vec2::new(self.w, self.h)
	}

	/// Returns the area of the [Rect].
	///
	/// This is typically equivalent to the product of [Rect::w] and [Rect::h].
	#[inline(always)]
	pub fn area(&self) -> f32 {
		self.w * self.h
	}

	/// Normalizes the [Rect] such that `w` and `h` are non-negative.
	///
	/// If either `w` or `h` is negative, the corresponding position is adjusted so that the
	/// rectangle retains the same bounds.
	///
	/// Returns a mutable reference to self to allow chaining.
	#[inline(always)]
	pub fn normalize(&mut self) -> &mut Self {
		self.x = self.left().min(self.right());
		self.y = self.top().min(self.bottom());
		self.w = self.w.abs();
		self.h = self.h.abs();
		self
	}

	/// Sets the position of the [Rect] to (`new_x`, `new_y`).
	#[inline(always)]
	pub fn reposition(&mut self, new_x: f32, new_y: f32) -> &mut Self {
		self.x = new_x;
		self.y = new_y;
		self
	}

	/// Sets the size of the [Rect] to (`new_w`, `new_h`).
	#[inline(always)]
	pub fn resize(&mut self, new_w: f32, new_h: f32) -> &mut Self {
		self.w = new_w;
		self.h = new_h;
		self
	}

	/// Returns whether the point at (`x`, `y`) is contained within the [Rect].
	#[inline(always)]
	pub fn contains_point(&self, x: f32, y: f32) -> bool {
		x >= self.x && x <= self.right() && y >= self.y && y <= self.bottom()
	}

	/// Returns whether the given [Rect] fits within the [Rect].
	#[inline(always)]
	pub fn contains_rect(&self, other: &Rect) -> bool {
		self.contains_point(other.x, other.y) && self.contains_point(other.right(), other.bottom())
	}

	// https://stackoverflow.com/questions/13390333/two-rectangles-intersection/44120056#44120056
	/// Returns whether the [Rect] overlaps with another [Rect].
	pub fn overlaps_with_rect(&self, other: &Rect) -> bool {
		!(self.right() < other.x
			|| other.right() < self.x
			|| self.bottom() < other.y
			|| other.bottom() < self.y)
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
	fn overlapping() {
		// half overlap
		let rect_a = Rect::from_xywh(0.0, 0.0, 1.0, 1.0);
		let rect_b = Rect::from_xywh(0.5, 0.5, 1.0, 1.0);
		assert!(rect_a.overlaps_with_rect(&rect_b));
		assert!(rect_b.overlaps_with_rect(&rect_a));

		// self overlapping
		let rect_a = Rect::from_xywh(0.0, 0.0, 1.0, 1.0);
		assert!(rect_a.overlaps_with_rect(&rect_a));

		// not overlapping
		let rect_a = Rect::from_xywh(0.0, 0.0, 1.0, 1.0);
		let rect_b = Rect::from_xywh(1.1, 1.1, 1.0, 1.0);
		assert!(!rect_a.overlaps_with_rect(&rect_b));
	}
}
