use crate::crop_range::CropRange;

pub trait Slice {
	fn slice(&self, range: impl CropRange) -> Self;
}

impl Slice for String {
	fn slice(&self, range: impl CropRange) -> Self {
		let mut slice = self.chars();

		if let Some(from) = range.get_bounds().0 {
			slice.nth(from - 1);
		}
		if let Some(to) = range.get_bounds().1 {
			let len = self.chars().count();
			slice.nth_back(len - to);
		}
		slice.as_str().to_owned()
	}
}

impl Slice for &str {
	fn slice(&self, range: impl CropRange) -> Self {
		let mut slice = self.chars();

		if let Some(from) = range.get_bounds().0 {
			slice.nth(from - 1);
		}
		if let Some(to) = range.get_bounds().1 {
			let len = self.chars().count();
			slice.nth_back(len - to);
		}
		slice.as_str()
	}
}

#[cfg(test)]
mod tests {
	use crate::slice_unchecked::Slice;

	#[test]
	fn test_core_left() {
		assert_eq!("es", "yes".slice(1..));
	}

	#[test]
	fn test_core_right() {
		assert_eq!("ye", "yes".slice(..3));
	}

	#[test]
	fn test_core_full() {
		assert_eq!("yes", "yes".slice(..));
	}

	#[test]
	fn test_core() {
		assert_eq!("e", "yes".slice(1..3));
	}
}