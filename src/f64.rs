use glam::DVec2;
use std::fmt;

/// A 2-dimensional rectangle.
///
/// The `width` and `height` of a rectangle are technically allowed to be negative, but it is heavily discouraged.
///
/// # Examples
/// ```
/// use glam_rect::DRect;
///
/// let new_rect = DRect::from_xywh(0.0, 0.0, 10.0, 10.0);
/// assert!(new_rect.contains_point(1.0, 1.0));
/// ```
#[derive(Clone, Copy, PartialEq)]
pub struct DRect {
	pub x: f64,
	pub y: f64,
	pub w: f64,
	pub h: f64,
}

impl DRect {
	/// Creates a new [DRect] with its top-left corner positioned at (`x`, `y`), with size (`w`, `h`).
	#[inline(always)]
	pub const fn from_xywh(x: f64, y: f64, w: f64, h: f64) -> Self {
		Self { x, y, w, h }
	}

	/// Creates a new [DRect] with its top-left corner positioned at (`left`, `top`)
	/// and its bottom-right corner positioned at (`right`, `bottom`).
	#[inline(always)]
	pub const fn from_ltrb(left: f64, top: f64, right: f64, bottom: f64) -> Self {
		Self {
			x: left,
			y: top,
			w: right - left,
			h: bottom - top,
		}
	}

	/// Returns the y-coordinate of the top edge of the [DRect].
	///
	/// This is typically equivalent to [DRect::y].
	///
	/// # Examples
	/// ```
	/// use glam_rect::DRect;
	///
	/// let new_rect = DRect::from_xywh(0.0, 0.0, 10.0, 10.0);
	/// assert_eq!(new_rect.top(), 0.0);
	/// ```
	#[inline(always)]
	pub fn top(&self) -> f64 {
		self.y
	}

	/// Returns the x-coordinate of the left edge of the [DRect].
	///
	/// This is typically equivalent to [DRect::x].
	///
	/// # Examples
	/// ```
	/// use glam_rect::DRect;
	///
	/// let new_rect = DRect::from_xywh(0.0, 0.0, 10.0, 10.0);
	/// assert_eq!(new_rect.left(), 0.0);
	/// ```
	#[inline(always)]
	pub fn left(&self) -> f64 {
		self.x
	}

	/// Returns the y-coordinate of the bottom edge of the [DRect].
	///
	/// This is typically equivalent to the sum of [DRect::y] and [DRect::h].
	///
	/// # Examples
	/// ```
	/// use glam_rect::DRect;
	///
	/// let new_rect = DRect::from_xywh(0.0, 0.0, 10.0, 10.0);
	/// assert_eq!(new_rect.bottom(), 10.0);
	/// ```
	#[inline(always)]
	pub fn bottom(&self) -> f64 {
		self.y + self.h
	}

	/// Returns the x-coordinate of the right edge of the [DRect].
	///
	/// This is typically equivalent to the sum of [DRect::x] and [DRect::w].
	///
	/// # Examples
	/// ```
	/// use glam_rect::DRect;
	///
	/// let new_rect = DRect::from_xywh(0.0, 0.0, 10.0, 10.0);
	/// assert_eq!(new_rect.right(), 10.0);
	/// ```
	#[inline(always)]
	pub fn right(&self) -> f64 {
		self.x + self.w
	}

	/// Returns the position of the top-left corner as a [DVec2].
	///
	/// # Examples
	/// ```
	/// use glam_rect::DRect;
	/// use glam::DVec2;
	///
	/// let new_rect = DRect::from_xywh(0.0, 0.0, 10.0, 10.0);
	/// assert_eq!(new_rect.top_left(), DVec2::new(0.0, 0.0));
	/// ```
	#[inline(always)]
	pub fn top_left(&self) -> DVec2 {
		DVec2::new(self.x, self.y)
	}

	/// Returns the position of the top-right corner as a [DVec2].
	///
	/// # Examples
	/// ```
	/// use glam_rect::DRect;
	/// use glam::DVec2;
	///
	/// let new_rect = DRect::from_xywh(0.0, 0.0, 10.0, 10.0);
	/// assert_eq!(new_rect.top_right(), DVec2::new(10.0, 0.0));
	/// ```
	#[inline(always)]
	pub fn top_right(&self) -> DVec2 {
		DVec2::new(self.x + self.w, self.y)
	}

	/// Returns the position of the bottom-left corner as a [DVec2].
	///
	/// # Examples
	/// ```
	/// use glam_rect::DRect;
	/// use glam::DVec2;
	///
	/// let new_rect = DRect::from_xywh(0.0, 0.0, 10.0, 10.0);
	/// assert_eq!(new_rect.bottom_left(), DVec2::new(0.0, 10.0));
	/// ```
	#[inline(always)]
	pub fn bottom_left(&self) -> DVec2 {
		DVec2::new(self.x, self.y + self.h)
	}

	/// Returns the position of the bottom-right corner as a [DVec2].
	///
	/// # Examples
	/// ```
	/// use glam_rect::DRect;
	/// use glam::DVec2;
	///
	/// let new_rect = DRect::from_xywh(0.0, 0.0, 10.0, 10.0);
	/// assert_eq!(new_rect.bottom_right(), DVec2::new(10.0, 10.0));
	/// ```
	#[inline(always)]
	pub fn bottom_right(&self) -> DVec2 {
		DVec2::new(self.x + self.w, self.y + self.h)
	}

	/// Returns the position of the center of the [DRect] as a [DVec2].
	///
	/// # Examples
	/// ```
	/// use glam_rect::DRect;
	/// use glam::DVec2;
	///
	/// let new_rect = DRect::from_xywh(0.0, 0.0, 10.0, 10.0);
	/// assert_eq!(new_rect.center(), DVec2::new(5.0, 5.0));
	/// ```
	#[inline(always)]
	pub fn center(&self) -> DVec2 {
		DVec2::new(self.x + self.w / 2.0, self.y + self.h / 2.0)
	}

	/// Returns the size of the [DRect] as a [DVec2] with components (`w`, `h`).
	///
	/// # Examples
	/// ```
	/// use glam_rect::DRect;
	/// use glam::DVec2;
	///
	/// let new_rect = DRect::from_xywh(0.0, 0.0, 10.0, 10.0);
	/// assert_eq!(new_rect.size(), DVec2::new(10.0, 10.0));
	/// ```
	#[inline(always)]
	pub fn size(&self) -> DVec2 {
		DVec2::new(self.w, self.h)
	}

	/// Returns the area of the [DRect].
	///
	/// This is typically equivalent to the product of [DRect::w] and [DRect::h].
	///
	/// # Examples
	/// ```
	/// use glam_rect::DRect;
	///
	/// let new_rect = DRect::from_xywh(0.0, 0.0, 10.0, 10.0);
	/// assert_eq!(new_rect.area(), 100.0);
	/// ```
	#[inline(always)]
	pub fn area(&self) -> f64 {
		self.w * self.h
	}

	/// Returns a normalized [DRect] such that `w` and `h` are non-negative.
	///
	/// If either `w` or `h` is negative, the corresponding position is adjusted so that the
	/// rectangle retains the same bounds.
	///
	/// # Examples
	/// ```
	/// use glam_rect::DRect;
	///
	/// let new_rect = DRect::from_xywh(0.0, 0.0, -10.0, 10.0);
	/// assert_eq!(new_rect.normalized(), DRect::from_xywh(-10.0, 0.0, 10.0, 10.0));
	/// ```
	#[inline(always)]
	pub fn normalized(&self) -> Self {
		Self {
			x: self.left().min(self.right()),
			y: self.top().min(self.bottom()),
			w: self.w.abs(),
			h: self.h.abs(),
		}
	}

	/// Returns a [DRect] with the same size and its top-left corner positioned at (`new_x`, `new_y`).
	///
	/// # Examples
	/// ```
	/// use glam_rect::DRect;
	///
	/// let new_rect = DRect::from_xywh(0.0, 0.0, 10.0, 10.0);
	/// assert_eq!(new_rect.repositioned(10.0, 5.0), DRect::from_xywh(10.0, 5.0, 10.0, 10.0));
	/// ```
	#[inline(always)]
	pub fn repositioned(&self, new_x: f64, new_y: f64) -> Self {
		Self {
			x: new_x,
			y: new_y,
			w: self.w,
			h: self.h,
		}
	}

	/// Returns a [DRect] with a size of (`new_w`, `new_h`) and the same position.
	///
	/// # Examples
	/// ```
	/// use glam_rect::DRect;
	///
	/// let new_rect = DRect::from_xywh(0.0, 0.0, 10.0, 10.0);
	/// assert_eq!(new_rect.resized(15.0, 5.0), DRect::from_xywh(0.0, 0.0, 15.0, 5.0));
	/// ```
	#[inline(always)]
	pub fn resized(&self, new_w: f64, new_h: f64) -> Self {
		Self {
			x: self.x,
			y: self.y,
			w: new_w,
			h: new_h,
		}
	}

	/// Returns whether the point at (`x`, `y`) is contained within the [DRect].
	///
	/// # Examples
	/// ```
	/// use glam_rect::DRect;
	///
	/// let new_rect = DRect::from_xywh(0.0, 0.0, 10.0, 10.0);
	/// assert!(new_rect.contains_point(5.0, 5.0));
	/// ```
	#[inline(always)]
	pub fn contains_point(&self, x: f64, y: f64) -> bool {
		x >= self.x && x <= self.right() && y >= self.y && y <= self.bottom()
	}

	/// Returns whether the given [DRect] fits within the [DRect].
	///
	/// # Examples
	/// ```
	/// use glam_rect::DRect;
	///
	/// let new_rect = DRect::from_xywh(0.0, 0.0, 10.0, 10.0);
	/// assert!(new_rect.contains_rect(&DRect::from_xywh(1.0, 1.0, 8.0, 8.0)));
	/// ```
	#[inline(always)]
	pub fn contains_rect(&self, other: &DRect) -> bool {
		self.contains_point(other.x, other.y) && self.contains_point(other.right(), other.bottom())
	}

	// solution: https://stackoverflow.com/questions/13390333/two-rectangles-intersection/44120056#44120056
	/// Returns whether the [DRect] overlaps with another [DRect].
	///
	/// # Examples
	/// ```
	/// use glam_rect::DRect;
	///
	/// let new_rect = DRect::from_xywh(0.0, 0.0, 10.0, 10.0);
	/// assert!(new_rect.overlaps_with_rect(&DRect::from_xywh(1.0, 1.0, 10.0, 10.0)));
	/// ```
	#[inline(always)]
	pub fn overlaps_with_rect(&self, other: &DRect) -> bool {
		!(self.right() < other.x
			|| other.right() < self.x
			|| self.bottom() < other.y
			|| other.bottom() < self.y)
	}
}

impl fmt::Display for DRect {
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

impl fmt::Debug for DRect {
	fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt.debug_tuple(stringify!(DRect))
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
		let new_rect = DRect::from_xywh(0.0, 0.0, 0.0, 0.0);
		assert_eq!(new_rect.x, 0.0);
		assert_eq!(new_rect.y, 0.0);
		assert_eq!(new_rect.w, 0.0);
		assert_eq!(new_rect.h, 0.0);

		let new_rect = DRect::from_xywh(-10.0, -293.39, 0.75, 10.3);
		assert_eq!(new_rect.x, -10.0);
		assert_eq!(new_rect.y, -293.39);
		assert_eq!(new_rect.w, 0.75);
		assert_eq!(new_rect.h, 10.3);
	}

	#[test]
	fn constructs_ltrb() {
		let new_rect = DRect::from_ltrb(0.0, 0.0, 0.0, 0.0);
		assert_eq!(new_rect.x, 0.0);
		assert_eq!(new_rect.y, 0.0);
		assert_eq!(new_rect.w, 0.0);
		assert_eq!(new_rect.h, 0.0);

		let new_rect = DRect::from_ltrb(0.0, 0.0, 100.0, 100.0);
		assert_eq!(new_rect.x, 0.0);
		assert_eq!(new_rect.y, 0.0);
		assert_eq!(new_rect.w, 100.0);
		assert_eq!(new_rect.h, 100.0);

		let new_rect = DRect::from_ltrb(50.0, 50.0, 100.0, 100.0);
		assert_eq!(new_rect.x, 50.0);
		assert_eq!(new_rect.y, 50.0);
		assert_eq!(new_rect.w, 50.0);
		assert_eq!(new_rect.h, 50.0);
	}

	#[test]
	fn overlapping() {
		// half overlap
		let rect_a = DRect::from_xywh(0.0, 0.0, 1.0, 1.0);
		let rect_b = DRect::from_xywh(0.5, 0.5, 1.0, 1.0);
		assert!(rect_a.overlaps_with_rect(&rect_b));
		assert!(rect_b.overlaps_with_rect(&rect_a));

		// self overlapping
		let rect_a = DRect::from_xywh(0.0, 0.0, 1.0, 1.0);
		assert!(rect_a.overlaps_with_rect(&rect_a));

		// not overlapping
		let rect_a = DRect::from_xywh(0.0, 0.0, 1.0, 1.0);
		let rect_b = DRect::from_xywh(1.1, 1.1, 1.0, 1.0);
		assert!(!rect_a.overlaps_with_rect(&rect_b));
	}
}
