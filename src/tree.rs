#[derive(Clone, Debug, Default)]
pub(crate) struct AATree<T> {
	root: AANode<T>
}

#[derive(Clone, Debug)]
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
			Self::Nil => Self::Node {
				level,
				content,
				left_child: l,
				right_child: r
			},
			Self::Node {
				level: l_level,
				content: l_content,
				left_child: a,
				right_child: b
			} => {
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
		($content:expr) => {
			AANode::new($content)
		};
		($content:expr => [$left:tt, $right:tt]) => {
			{
				let _left = tree!(@internal $left);
				let _right = tree!(@internal $right);
				// TODO properly compute the level
				let _level = _left.level() + 1;
				AANode::Node {
					level: _level,
					content: $content,
					left_child: Box::new(_left),
					right_child: Box::new(_right)
				}
			}
		};
		(@internal ($content:expr => [$left:tt, $right:tt])) => {
			tree!($content => [$left, $right])
		};
		(@internal $inner:tt) => {
			tree!($inner)
		};
	}

	#[test]
	fn test_skew() {
		let root = tree!('T' => [('L' => ['A', 'B']), 'R']);
		let skewed = root.skew();
		assert_eq!(skewed.content(), Some(&'L'));
	}
}
