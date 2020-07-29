#[derive(Clone, Debug, PartialEq)]
pub(super) struct AANode<T> {
	level: u8,
	content: T,
	left: Option<Box<AANode<T>>>,
	right: Option<Box<AANode<T>>>
}

impl<T> From<T> for AANode<T> {
	fn from(content: T) -> Self {
		Self {
			level: 1,
			content,
			left: None,
			right: None
		}
	}
}

trait Skew {
	type Node;
	fn skew(self) -> Self::Node;
}

impl<T> Skew for Box<AANode<T>> {
	type Node = Self;

	/// ```none
	///   L <--- S           S ---> T
	///  / \      \     =>  /      / \
	/// A   B      R       A      B   R
	/// ```
	fn skew(mut self) -> Self {
		let mut l = match self.left {
			Some(l) if l.level == self.level => l,
			_ => return self
		};

		// take the B node from L
		let b = l.right.take();
		// place B as our left child, removing L
		self.left = b;
		// add our node (T) as the right child of L
		l.right = Some(self);
		// the new node is L
		l
	}
}

trait Split {
	type Node;
	fn split(self) -> Self::Node;
}

impl<T> Split for Box<AANode<T>> {
	type Node = Self;

	/// ```none
	///   S --> R --> X          R
	///  /     /          =>    / \
	/// A     B                T   X
	///                       / \
	///                      A   B
	/// ```
	fn split(mut self) -> Self {
		let mut r = match self.right.take() {
			Some(r) => r,
			None => return self
		};

		// remove the B node from R if all are on the same level
		let b = match r.left.take() {
			Some(b) if b.right.as_ref().map(|x| x.level).unwrap_or(0) == self.level => b,
			_ => return self
		};

		// place B as our new right child
		self.right = Some(b);

		// attach our node (T) to R and increase the level of R
		r.left = Some(self);
		r.level += 1;

		// R is the new node
		r
	}
}

#[cfg(all(test, not(feature = "benchmark")))]
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
			AANode::from($content)
		};
		($content:expr => [$level:expr, $left:tt, $right:tt]) => {
			{
				let _left = tree!(@internal $left);
				let _right = tree!(@internal $right);
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
			assert!(root.insert(*content));
		}
		let expected = tree!('D' => [3, ('B' => [2, 'A', 'C']), ('F' => [2, 'E', 'G'])]);
		assert_eq!(root, expected);
	}

	#[test]
	fn test_insert_smaller() {
		let mut root = tree!();
		for content in ['Z', 'Y', 'X', 'W', 'V'].iter() {
			assert!(root.insert(*content));
		}
		let expected = tree!('W' => [2, 'V', ('Y' => [2, 'X', 'Z'])]);
		assert_eq!(root, expected);
	}

	#[test]
	fn test_insert_multiple() {
		let mut root = tree!();
		for content in ['A', 'A'].iter() {
			root.insert(*content);
		}
		let expected = tree!('A');
		assert_eq!(root, expected);
	}

	// ### TEST REMOVE ###

	#[test]
	fn test_remove_successor() {
		let mut root = tree!('B' => [1, Nil, 'C']);
		println!("Input:  `{:?}`", root);
		let removed = root.remove(&'B');
		let expected = tree!('C');
		assert_eq!(removed, Some('B'));
		assert_eq!(root, expected);
	}

	#[test]
	fn test_remove_predecessor() {
		let mut root = tree!('B' => [2, 'A', 'C']);
		println!("Input:  `{:?}`", root);
		let removed = root.remove(&'B');
		let expected = tree!('A' => [1, Nil, 'C']);
		assert_eq!(removed, Some('B'));
		assert_eq!(root, expected);
	}

	#[test]
	fn test_remove_complex() {
		// example taken from https://web.eecs.umich.edu/~sugih/courses/eecs281/f11/lectures/12-AAtrees+Treaps.pdf
		let mut root =
			tree!(30 => [3, (15 => [2, 5, 20]), (70 => [3, (50 => [2, 35, (60 => [2, 55, 65])]), (85 => [2, 80, 90])])]);
		println!("Input:  `{:?}`", root);
		let removed = root.remove(&5);
		let expected =
			tree!(50 => [3, (30 => [2, (15 => [1, Nil, 20]), 35]), (70 => [3, (60 => [2, 55, 65]), (85 => [2, 80, 90])])]);
		assert_eq!(removed, Some(5));
		assert_eq!(root, expected);
	}
}
