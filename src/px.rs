pub struct PxRect {
	pub x: u32,
	pub y: u32,
	pub w: u32,
	pub h: u32,
}

impl PxRect {
	/// Creates a new [Rect] with its top-left corner positioned at (`x`, `y`), with size (`w`, `h`).
	#[inline(always)]
	pub const fn from_xywh(x: u32, y: u32, w: u32, h: u32) -> Self {
		Self { x, y, w, h }
	}

	/// Creates a new [Rect] with its top-left corner positioned at (`left`, `top`)
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
}
