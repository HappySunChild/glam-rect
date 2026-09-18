use glam::Vec2;

#[derive(Clone, Copy, PartialEq)]
pub struct Rect {
	pub x: f32,
	pub y: f32,
	pub w: f32,
	pub h: f32,
}

impl Rect {
	/// Creates a new [Rect] with its top-left corner positioned at (`x`, `y`), with size (`w`, `h`).
	pub fn from_xywh(x: f32, y: f32, w: f32, h: f32) -> Self {
		Self { x, y, w, h }
	}

	/// Creates a new [Rect] with its top-left corner positioned at (`left`, `top`)
	/// and its bottom-right corner positioned at (`right`, `bottom`).
	pub fn from_ltrb(left: f32, top: f32, right: f32, bottom: f32) -> Self {
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

	/// Returns whether the specified `point` is contained within the [Rect].
	#[inline(always)]
	pub fn contains(&self, point: Vec2) -> bool {
		point.x >= self.x
			&& point.x <= self.right()
			&& point.y >= self.y
			&& point.y <= self.bottom()
	}
}
