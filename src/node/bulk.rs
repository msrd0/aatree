use super::{AANode, Node};
use alloc::boxed::Box;

impl<T: Ord> AANode<T> {
	/// Insert from a **sorted** array. **Destroys any previous data in `self`.**
	pub(crate) fn bulk_insert<I>(&mut self, mut data: I)
	where
		I: Iterator<Item = T> + ExactSizeIterator
	{
		let len = data.len();
		self.bulk_insert_len(&mut data, len);
	}

	fn bulk_insert_len<I>(&mut self, data: &mut I, len: usize)
	where
		I: Iterator<Item = T>
	{
		match len {
			0 => {
				self.0 = None;
			},
			1 => {
				*self = data.next().unwrap().into();
			},

			len => {
				let root_idx = (len - 1) / 2;
				let mut left = Self::new();
				left.bulk_insert_len(data, root_idx);
				let root_val = data.next().unwrap();
				let mut right = Self::new();
				right.bulk_insert_len(data, len - root_idx - 1);

				self.0 = Some(Box::new(Node {
					level: left.level() + 1,
					content: root_val,
					left_child: left,
					right_child: right
				}));
			}
		}
	}
}
