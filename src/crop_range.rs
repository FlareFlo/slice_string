use std::ops::{Range, RangeFrom, RangeFull, RangeTo};

pub trait CropRange {
	fn get_bounds(&self) -> (Option<usize>, Option<usize>);
}

impl CropRange for Range<usize> {
	fn get_bounds(&self) -> (Option<usize>, Option<usize>) {
		(Some(self.start), Some(self.end))
	}
}

impl CropRange for RangeTo<usize> {
	fn get_bounds(&self) -> (Option<usize>, Option<usize>) {
		(None, Some(self.end))
	}
}

impl CropRange for RangeFrom<usize> {
	fn get_bounds(&self) -> (Option<usize>, Option<usize>) {
		(Some(self.start), None)
	}
}

impl CropRange for RangeFull {
	fn get_bounds(&self) -> (Option<usize>, Option<usize>) {
		(None, None)
	}
}

#[cfg(test)]
mod tests {
	use crate::crop_range::CropRange;

	#[test]
	fn range() {
		assert_eq!((Some(1), Some(3)), (1..3_usize).get_bounds());
	}
}