use super::AANode;

impl<T: Ord> AANode<T> {
	/// Insert a new node with `content` into the tree. If a node with this value already exist,
	/// nothing will be inserted, and `false` will be returned.
	pub fn insert(&mut self, content: T) -> bool {
		let inserted = self.bst_insert(content);
		if inserted {
			let mut node = std::mem::replace(self, Self::Nil);
			node = node.skew().split();
			*self = node;
		}
		inserted
	}

	/// Simple unbalanced BST insert.
	fn bst_insert(&mut self, new: T) -> bool {
		match self {
			Self::Nil => {
				*self = new.into();
				true
			},
			Self::Node { content, left_child, .. } if &new < content => left_child.insert(new),
			Self::Node {
				content, right_child, ..
			} if &new > content => right_child.insert(new),
			_ => false
		}
	}
}
