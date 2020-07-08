#[derive(Clone, Debug, Default)]
pub(crate) struct AATree<T> {
	root: AANode<T>
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum AANode<T> {
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
}
