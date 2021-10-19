use super::{AANode, Node};

impl<T: Ord> AANode<T> {
	/// Insert a new node with `content` into the tree. If a node with this value already exist,
	/// nothing will be inserted, and `false` will be returned.
	pub fn insert(&mut self, content: T) -> bool {
		let inserted = self.bst_insert(content);
		if inserted {
			let mut node = self.take();
			node = node.skew().split();
			*self = node;
		}
		inserted
	}

	/// Simple unbalanced BST insert.
	fn bst_insert(&mut self, new: T) -> bool {
		match self.as_mut() {
			None => {
				*self = new.into();
				true
			},
			Some(Node { content, left_child, .. }) if &new < content => left_child.insert(new),
			Some(Node {
				content, right_child, ..
			}) if &new > content => right_child.insert(new),
			_ => false
		}
	}
}
