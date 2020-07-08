/// An AA Tree.
#[derive(Clone, Debug)]
pub struct AATree<T> {
	root: AANode<T>
}

impl<T> AATree<T> {
	pub fn new() -> Self {
		Self { root: AANode::Nil }
	}
}

impl<T> Default for AATree<T> {
	fn default() -> Self {
		Self::new()
	}
}

impl<T: Ord> AATree<T> {
	/// Add a value to this tree. If the value already exists in the tree, nothing
	/// is inserted and `false` is returned.
	pub fn insert(&mut self, value: T) -> bool {
		let root = std::mem::replace(&mut self.root, AANode::Nil);
		let (root, inserted) = root.insert(value);
		self.root = root;
		inserted
	}
}

#[derive(Clone, Debug, PartialEq)]
enum AANode<T> {
	Nil,
	Node {
		level: u8,
		content: T,
		left_child: Box<AANode<T>>,
		right_child: Box<AANode<T>>
	}
}

impl<T> Default for AANode<T> {
	fn default() -> Self {
		Self::Nil
	}
}

impl<T> AANode<T> {
	fn new(content: T) -> Self {
		Self::Node {
			level: 1,
			content,
			left_child: Box::new(Self::Nil),
			right_child: Box::new(Self::Nil)
		}
	}

	fn level(&self) -> u8 {
		match self {
			Self::Nil => 0,
			Self::Node { level, .. } => *level
		}
	}

	fn content(&self) -> Option<&T> {
		match self {
			Self::Nil => None,
			Self::Node { content, .. } => Some(content)
		}
	}

	/// ```none
	///   L <--- S           S ---> T
	///  / \      \     =>  /      / \
	/// A   B      R       A      B   R
	/// ```
	fn skew(self) -> Self {
		match self {
			Self::Nil => self,
			Self::Node {
				level,
				content,
				left_child: l,
				right_child: r
			} => Self::skew_impl(level, content, l, r)
		}
	}

	fn skew_impl(level: u8, content: T, l: Box<Self>, r: Box<Self>) -> Self {
		match *l {
			Self::Node {
				level: l_level,
				content: l_content,
				left_child: a,
				right_child: b
			} if l_level == level => {
				let t = Self::Node {
					level,
					content,
					left_child: b,
					right_child: r
				};
				Self::Node {
					level: l_level,
					content: l_content,
					left_child: a,
					right_child: Box::new(t)
				}
			},
			_ => Self::Node {
				level,
				content,
				left_child: l,
				right_child: r
			}
		}
	}

	/// ```none
	///   S --> R --> X          R
	///  /     /          =>    / \
	/// A     B                T   X
	///                       / \
	///                      A   B
	/// ```
	fn split(self) -> Self {
		match self {
			Self::Node {
				level,
				content,
				left_child: a,
				right_child: r
			} if !matches!(*r, Self::Nil) => Self::split_impl(level, content, a, r),
			_ => self
		}
	}

	fn split_impl(level: u8, content: T, a: Box<Self>, r: Box<Self>) -> Self {
		match *r {
			Self::Node {
				level: r_level,
				content: r_content,
				left_child: b,
				right_child: x
			} if level == x.level() => {
				let t = Self::Node {
					level,
					content,
					left_child: a,
					right_child: b
				};
				Self::Node {
					level: r_level + 1,
					content: r_content,
					left_child: Box::new(t),
					right_child: x
				}
			},
			_ => Self::Node {
				level,
				content,
				left_child: a,
				right_child: r
			}
		}
	}
}

impl<T: Ord> AANode<T> {
	/// Insert a new node with `content` into the tree. If a node with this value already exist,
	/// nothing will be inserted. Returns a pair of the new node and a boolean whether a new node
	/// was inserted.
	fn insert(self, content: T) -> (Self, bool) {
		let (mut node, inserted) = self.bst_insert(content);
		if inserted {
			node = node.skew().split();
		}
		(node, inserted)
	}

	/// Simple unbalanced BST insert.
	fn bst_insert(self, new: T) -> (Self, bool) {
		match self {
			Self::Nil => (Self::new(new), true),
			Self::Node {
				level,
				content,
				left_child,
				right_child
			} => {
				if new < content {
					let (left_child, inserted) = left_child.insert(new);
					(
						Self::Node {
							level,
							content,
							left_child: Box::new(left_child),
							right_child
						},
						inserted
					)
				} else if new > content {
					let (right_child, inserted) = right_child.insert(new);
					(
						Self::Node {
							level,
							content,
							left_child,
							right_child: Box::new(right_child)
						},
						inserted
					)
				} else {
					(
						Self::Node {
							level,
							content,
							left_child,
							right_child
						},
						false
					)
				}
			},
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	macro_rules! tree {
		() => {
			AANode::Nil
		};
		(Nil) => {
			AANode::Nil
		};
		($content:expr) => {
			AANode::new($content)
		};
		($content:expr => [$level:expr, $left:tt, $right:tt]) => {
			{
				let _left = tree!(@internal $left);
				let _right = tree!(@internal $right);
				// TODO properly compute the level
				AANode::Node {
					level: $level,
					content: $content,
					left_child: Box::new(_left),
					right_child: Box::new(_right)
				}
			}
		};
		(@internal ($content:expr => [$level:expr, $left:tt, $right:tt])) => {
			tree!($content => [$level, $left, $right])
		};
		(@internal $inner:tt) => {
			tree!($inner)
		};
	}

	// ### TEST SKEW ###

	#[test]
	fn test_skew_nil() {
		let root: AANode<char> = tree!();
		println!("Input: {:?}", root);
		let skewed = root.skew();
		let expected = tree!();
		assert_eq!(skewed, expected);
	}

	#[test]
	fn test_skew_leaf() {
		let root = tree!('T');
		println!("Input: {:?}", root);
		let skewed = root.skew();
		let expected = tree!('T');
		assert_eq!(skewed, expected);
	}

	#[test]
	fn test_skew_simple() {
		let root = tree!('T' => [2, ('L' => [2, Nil, Nil]), 'R']);
		println!("Input: {:?}", root);
		let skewed = root.skew();
		let expected = tree!('L' => [2, Nil, ('T' => [2, Nil, 'R'])]);
		assert_eq!(skewed, expected);
	}

	#[test]
	fn test_skew_full() {
		let root = tree!('T' => [2, ('L' => [2, 'A', 'B']), 'R']);
		println!("Input: {:?}", root);
		let skewed = root.skew();
		let expected = tree!('L' => [2, 'A', ('T' => [2, 'B', 'R'])]);
		assert_eq!(skewed, expected);
	}

	// ### TEST SPLIT ###

	#[test]
	fn test_split_nil() {
		let root: AANode<char> = tree!();
		println!("Input: {:?}", root);
		let splitted = root.split();
		let expected = tree!();
		assert_eq!(splitted, expected);
	}

	#[test]
	fn test_split_leaf() {
		let root = tree!('T');
		println!("Input: {:?}", root);
		let splitted = root.split();
		let expected = tree!('T');
		assert_eq!(splitted, expected);
	}

	#[test]
	fn test_split_good_tree() {
		let root = tree!('T' => [2, 'A', ('R' => [2, 'B', 'X'])]);
		println!("Input: {:?}", root);
		let splitted = root.split();
		let expected = tree!('T' => [2, 'A', ('R' => [2, 'B', 'X'])]);
		assert_eq!(splitted, expected);
	}

	#[test]
	fn test_split_bad_tree() {
		let root = tree!('T' => [2, 'A', ('R' => [2, 'B', ('X' => [2, 'Y', 'Z'])])]);
		println!("Input: {:?}", root);
		let splitted = root.split();
		let expected = tree!('R' => [3, ('T' => [2, 'A', 'B']), ('X' => [2, 'Y', 'Z'])]);
		assert_eq!(splitted, expected);
	}

	// ### TEST INSERT ###

	#[test]
	fn test_insert_greater() {
		let mut root = tree!();
		for content in ['A', 'B', 'C', 'D', 'E', 'F', 'G'].iter() {
			root = root.insert(*content).0;
		}
		let expected = tree!('D' => [3, ('B' => [2, 'A', 'C']), ('F' => [2, 'E', 'G'])]);
		assert_eq!(root, expected);
	}

	#[test]
	fn test_insert_smaller() {
		let mut root = tree!();
		for content in ['Z', 'Y', 'X', 'W', 'V'].iter() {
			root = root.insert(*content).0;
		}
		let expected = tree!('W' => [2, 'V', ('Y' => [2, 'X', 'Z'])]);
		assert_eq!(root, expected);
	}

	#[test]
	fn test_insert_multiple() {
		let mut root = tree!();
		for content in ['A', 'A'].iter() {
			root = root.insert(*content).0;
		}
		let expected = tree!('A');
		assert_eq!(root, expected);
	}
}
