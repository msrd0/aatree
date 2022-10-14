use super::{AANode, Node};
use alloc::boxed::Box;

impl<T: Ord> AANode<T> {
	/// Create tree from a **sorted** array.
	pub(crate) fn from_sorted_data<I>(mut data: I) -> Self
	where
		I: Iterator<Item = T> + ExactSizeIterator
	{
		let len = data.len();
		Self::from_sorted_data_impl(&mut data, len)
	}

	fn from_sorted_data_impl<I>(data: &mut I, len: usize) -> Self
	where
		I: Iterator<Item = T>
	{
		match len {
			0 => Self::new(),
			1 => data.next().unwrap().into(),

			len => {
				let root_idx = (len - 1) / 2;
				let left_child = Self::from_sorted_data_impl(data, root_idx);
				let content = data.next().unwrap();
				let right_child = Self::from_sorted_data_impl(data, len - root_idx - 1);

				Self(Some(Box::new(Node {
					level: left_child.level() + 1,
					content,
					left_child,
					right_child
				})))
			}
		}
	}
}
