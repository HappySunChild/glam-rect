use glam::UVec2;
use std::fmt;

/// A 2-dimensional rectangle.
///
/// # Examples
/// ```
/// use glam_rect::PxRect;
///
/// let new_rect = PxRect::from_xywh(0, 0, 10, 10);
/// assert!(new_rect.contains_point(1, 1));
/// ```
#[derive(Clone, Copy, PartialEq)]
pub struct PxRect {
	pub x: u32,
	pub y: u32,
	pub w: u32,
	pub h: u32,
}

impl PxRect {
	/// Creates a new [PxRect] with its top-left corner positioned at (`x`, `y`), with size (`w`, `h`).
	#[inline(always)]
	pub const fn from_xywh(x: u32, y: u32, w: u32, h: u32) -> Self {
		Self { x, y, w, h }
	}

	/// Creates a new [PxRect] with its top-left corner positioned at (`left`, `top`)
	/// and its bottom-right corner positioned at (`right`, `bottom`).
	#[inline(always)]
	pub const fn from_ltrb(left: u32, top: u32, right: u32, bottom: u32) -> Self {
		Self {
			x: left,
			y: top,
			w: right - left,
			h: bottom - top,
		}
	}

	/// Returns the y-coordinate of the top edge of the [PxRect].
	///
	/// This is typically equivalent to [PxRect::y].
	///
	/// # Examples
	/// ```
	/// use glam_rect::PxRect;
	///
	/// let new_rect = PxRect::from_xywh(0, 0, 10, 10);
	/// assert_eq!(new_rect.top(), 0);
	/// ```
	#[inline(always)]
	pub fn top(&self) -> u32 {
		self.y
	}

	/// Returns the x-coordinate of the left edge of the [PxRect].
	///
	/// This is typically equivalent to [PxRect::x].
	///
	/// # Examples
	/// ```
	/// use glam_rect::PxRect;
	///
	/// let new_rect = PxRect::from_xywh(0, 0, 10, 10);
	/// assert_eq!(new_rect.left(), 0);
	/// ```
	#[inline(always)]
	pub fn left(&self) -> u32 {
		self.x
	}

	/// Returns the y-coordinate of the bottom edge of the [PxRect].
	///
	/// This is typically equivalent to the sum of [PxRect::y] and [PxRect::h].
	///
	/// # Examples
	/// ```
	/// use glam_rect::PxRect;
	///
	/// let new_rect = PxRect::from_xywh(0, 0, 10, 10);
	/// assert_eq!(new_rect.bottom(), 10);
	/// ```
	#[inline(always)]
	pub fn bottom(&self) -> u32 {
		self.y + self.h
	}

	/// Returns the x-coordinate of the right edge of the [PxRect].
	///
	/// This is typically equivalent to the sum of [PxRect::x] and [PxRect::w].
	///
	/// # Examples
	/// ```
	/// use glam_rect::PxRect;
	///
	/// let new_rect = PxRect::from_xywh(0, 0, 10, 10);
	/// assert_eq!(new_rect.right(), 10);
	/// ```
	#[inline(always)]
	pub fn right(&self) -> u32 {
		self.x + self.w
	}

	/// Returns the position of the top-left corner as a [UVec2].
	///
	/// # Examples
	/// ```
	/// use glam_rect::PxRect;
	/// use glam::UVec2;
	///
	/// let new_rect = PxRect::from_xywh(0, 0, 10, 10);
	/// assert_eq!(new_rect.top_left(), UVec2::new(0, 0));
	/// ```
	#[inline(always)]
	pub fn top_left(&self) -> UVec2 {
		UVec2::new(self.x, self.y)
	}

	/// Returns the position of the top-right corner as a [UVec2].
	///
	/// # Examples
	/// ```
	/// use glam_rect::PxRect;
	/// use glam::UVec2;
	///
	/// let new_rect = PxRect::from_xywh(0, 0, 10, 10);
	/// assert_eq!(new_rect.top_right(), UVec2::new(10, 0));
	/// ```
	#[inline(always)]
	pub fn top_right(&self) -> UVec2 {
		UVec2::new(self.x + self.w, self.y)
	}

	/// Returns the position of the bottom-left corner as a [UVec2].
	///
	/// # Examples
	/// ```
	/// use glam_rect::PxRect;
	/// use glam::UVec2;
	///
	/// let new_rect = PxRect::from_xywh(0, 0, 10, 10);
	/// assert_eq!(new_rect.bottom_left(), UVec2::new(0, 10));
	/// ```
	#[inline(always)]
	pub fn bottom_left(&self) -> UVec2 {
		UVec2::new(self.x, self.y + self.h)
	}

	/// Returns the position of the bottom-right corner as a [UVec2].
	///
	/// # Examples
	/// ```
	/// use glam_rect::PxRect;
	/// use glam::UVec2;
	///
	/// let new_rect = PxRect::from_xywh(0, 0, 10, 10);
	/// assert_eq!(new_rect.bottom_right(), UVec2::new(10, 10));
	/// ```
	#[inline(always)]
	pub fn bottom_right(&self) -> UVec2 {
		UVec2::new(self.x + self.w, self.y + self.h)
	}

	/// Returns the position of the center of the [PxRect] as a [UVec2].
	///
	/// # Examples
	/// ```
	/// use glam_rect::PxRect;
	/// use glam::UVec2;
	///
	/// let new_rect = PxRect::from_xywh(0, 0, 10, 10);
	/// assert_eq!(new_rect.center(), UVec2::new(5, 5));
	/// ```
	#[inline(always)]
	pub fn center(&self) -> UVec2 {
		UVec2::new(self.x + self.w / 2, self.y + self.h / 2)
	}

	/// Returns the size of the [PxRect] as a [UVec2] with components (`w`, `h`).
	///
	/// # Examples
	/// ```
	/// use glam_rect::PxRect;
	/// use glam::UVec2;
	///
	/// let new_rect = PxRect::from_xywh(0, 0, 10, 10);
	/// assert_eq!(new_rect.size(), UVec2::new(10, 10));
	/// ```
	#[inline(always)]
	pub fn size(&self) -> UVec2 {
		UVec2::new(self.w, self.h)
	}

	/// Returns the area of the [PxRect].
	///
	/// This is typically equivalent to the product of [PxRect::w] and [PxRect::h].
	///
	/// # Examples
	/// ```
	/// use glam_rect::PxRect;
	///
	/// let new_rect = PxRect::from_xywh(0, 0, 10, 10);
	/// assert_eq!(new_rect.area(), 100);
	/// ```
	#[inline(always)]
	pub fn area(&self) -> u32 {
		self.w * self.h
	}

	/// Returns a [PxRect] with the same size and its top-left corner positioned at (`new_x`, `new_y`).
	///
	/// # Examples
	/// ```
	/// use glam_rect::PxRect;
	///
	/// let new_rect = PxRect::from_xywh(0, 0, 10, 10);
	/// assert_eq!(new_rect.repositioned(10, 5), PxRect::from_xywh(10, 5, 10, 10));
	/// ```
	#[inline(always)]
	pub fn repositioned(&self, new_x: u32, new_y: u32) -> Self {
		Self {
			x: new_x,
			y: new_y,
			w: self.w,
			h: self.h,
		}
	}

	/// Returns a [PxRect] with a size of (`new_w`, `new_h`) and the same position.
	///
	/// # Examples
	/// ```
	/// use glam_rect::PxRect;
	///
	/// let new_rect = PxRect::from_xywh(0, 0, 10, 10);
	/// assert_eq!(new_rect.resized(15, 5), PxRect::from_xywh(0, 0, 15, 5));
	/// ```
	#[inline(always)]
	pub fn resized(&self, new_w: u32, new_h: u32) -> Self {
		Self {
			x: self.x,
			y: self.y,
			w: new_w,
			h: new_h,
		}
	}

	/// Returns whether the point at (`x`, `y`) is contained within the [PxRect].
	///
	/// # Examples
	/// ```
	/// use glam_rect::PxRect;
	///
	/// let new_rect = PxRect::from_xywh(0, 0, 10, 10);
	/// assert!(new_rect.contains_point(5, 5));
	/// ```
	#[inline(always)]
	pub fn contains_point(&self, x: u32, y: u32) -> bool {
		x >= self.x && x <= self.right() && y >= self.y && y <= self.bottom()
	}

	/// Returns whether the given [PxRect] fits within the [PxRect].
	///
	/// # Examples
	/// ```
	/// use glam_rect::PxRect;
	///
	/// let new_rect = PxRect::from_xywh(0, 0, 10, 10);
	/// assert!(new_rect.contains_rect(&PxRect::from_xywh(1, 1, 8, 8)));
	/// ```
	#[inline(always)]
	pub fn contains_rect(&self, other: &PxRect) -> bool {
		self.contains_point(other.x, other.y) && self.contains_point(other.right(), other.bottom())
	}

	// solution: https://stackoverflow.com/questions/13390333/two-rectangles-intersection/44120056#44120056
	/// Returns whether the [PxRect] overlaps with another [PxRect].
	///
	/// # Examples
	/// ```
	/// use glam_rect::PxRect;
	///
	/// let new_rect = PxRect::from_xywh(0, 0, 10, 10);
	/// assert!(new_rect.overlaps_with_rect(&PxRect::from_xywh(1, 1, 10, 10)));
	/// ```
	#[inline(always)]
	pub fn overlaps_with_rect(&self, other: &PxRect) -> bool {
		!(self.right() < other.x
			|| other.right() < self.x
			|| self.bottom() < other.y
			|| other.bottom() < self.y)
	}
}

impl fmt::Display for PxRect {
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

impl fmt::Debug for PxRect {
	fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt.debug_tuple(stringify!(PxRect))
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
		let new_rect = PxRect::from_xywh(0, 0, 0, 0);
		assert_eq!(new_rect.x, 0);
		assert_eq!(new_rect.y, 0);
		assert_eq!(new_rect.w, 0);
		assert_eq!(new_rect.h, 0);

		let new_rect = PxRect::from_xywh(10, 293, 0, 10);
		assert_eq!(new_rect.x, 10);
		assert_eq!(new_rect.y, 293);
		assert_eq!(new_rect.w, 0);
		assert_eq!(new_rect.h, 10);
	}

	#[test]
	fn constructs_ltrb() {
		let new_rect = PxRect::from_ltrb(0, 0, 0, 0);
		assert_eq!(new_rect.x, 0);
		assert_eq!(new_rect.y, 0);
		assert_eq!(new_rect.w, 0);
		assert_eq!(new_rect.h, 0);

		let new_rect = PxRect::from_ltrb(0, 0, 100, 100);
		assert_eq!(new_rect.x, 0);
		assert_eq!(new_rect.y, 0);
		assert_eq!(new_rect.w, 100);
		assert_eq!(new_rect.h, 100);

		let new_rect = PxRect::from_ltrb(50, 50, 100, 100);
		assert_eq!(new_rect.x, 50);
		assert_eq!(new_rect.y, 50);
		assert_eq!(new_rect.w, 50);
		assert_eq!(new_rect.h, 50);
	}

	#[test]
	fn overlapping() {
		// half overlap
		let rect_a = PxRect::from_xywh(0, 0, 2, 2);
		let rect_b = PxRect::from_xywh(1, 1, 2, 2);
		assert!(rect_a.overlaps_with_rect(&rect_b));
		assert!(rect_b.overlaps_with_rect(&rect_a));

		// self overlapping
		let rect_a = PxRect::from_xywh(0, 0, 1, 1);
		assert!(rect_a.overlaps_with_rect(&rect_a));

		// not overlapping
		let rect_a = PxRect::from_xywh(0, 0, 1, 1);
		let rect_b = PxRect::from_xywh(2, 2, 1, 1);
		assert!(!rect_a.overlaps_with_rect(&rect_b));
	}
}
