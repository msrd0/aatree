use super::*;

macro_rules! tree {
		() => {
			AANode::new()
		};
		(Nil) => {
			AANode::new()
		};
		($content:expr) => {
			AANode::from($content)
		};
		($content:expr => [$level:expr, $left:tt, $right:tt]) => {
			{
				let _left = tree!(@internal $left);
				let _right = tree!(@internal $right);
				AANode(Some(Box::new(Node {
					level: $level,
					content: $content,
					left_child: _left,
					right_child: _right
				})))
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

mod skew {
	use super::*;

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

// ### TEST SPLIT ###

mod split {
	use super::*;

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
}

// ### TEST INSERT ###

mod insert {
	use super::*;

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
}

// ### TEST REMOVE ###

mod remove {
	use super::*;

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
		let mut root = tree!(30 => [3, (15 => [2, 5, 20]), (70 => [3, (50 => [2, 35, (60 => [2, 55, 65])]), (85 => [2, 80, 90])])]);
		println!("Input:  `{:?}`", root);
		let removed = root.remove(&5);
		let expected = tree!(50 => [3, (30 => [2, (15 => [1, Nil, 20]), 35]), (70 => [3, (60 => [2, 55, 65]), (85 => [2, 80, 90])])]);
		assert_eq!(removed, Some(5));
		assert_eq!(root, expected);
	}
}

// ### TEST BULK INSERT ###

mod bulk_insert {
	use super::*;

	fn tree_from<I>(data: I) -> AANode<I::Item>
	where
		I: IntoIterator,
		<I as IntoIterator>::Item: Ord,
		<I as IntoIterator>::IntoIter: ExactSizeIterator
	{
		AANode::from_sorted_data(data.into_iter())
	}

	#[test]
	fn test_empty() {
		let tree: AANode<i32> = tree_from([]);
		let expected = tree!();
		assert_eq!(tree, expected);
	}

	#[test]
	fn test_len_1() {
		let tree: AANode<i32> = tree_from([1]);
		let expected = tree!(1);
		assert_eq!(tree, expected);
	}

	#[test]
	fn test_len_2() {
		let tree: AANode<i32> = tree_from([1, 2]);
		let expected = tree!(1 => [1, Nil, 2]);
		assert_eq!(tree, expected);
	}

	#[test]
	fn test_len_3() {
		let tree: AANode<i32> = tree_from([1, 2, 3]);
		let expected = tree!(2 => [2, 1, 3]);
		assert_eq!(tree, expected);
	}

	#[test]
	fn test_len_4() {
		let tree: AANode<i32> = tree_from([1, 2, 3, 4]);
		let expected = tree!(2 => [2, 1, (3 => [1, Nil, 4])]);
		assert_eq!(tree, expected);
	}

	#[test]
	fn test_len_5() {
		let tree: AANode<i32> = tree_from([1, 2, 3, 4, 5]);
		let expected = tree!(3 => [2, (1 => [1, Nil, 2]), (4 => [1, Nil, 5])]);
		assert_eq!(tree, expected);
	}

	#[test]
	fn test_len_6() {
		let tree: AANode<i32> = tree_from([1, 2, 3, 4, 5, 6]);
		let expected = tree!(3 => [2, (1 => [1, Nil, 2]), (5 => [2, 4, 6])]);
		assert_eq!(tree, expected);
	}

	#[test]
	fn test_len_7() {
		let tree: AANode<i32> = tree_from([1, 2, 3, 4, 5, 6, 7]);
		let expected = tree!(4 => [3, (2 => [2, 1, 3]), (6 => [2, 5, 7])]);
		assert_eq!(tree, expected);
	}

	#[test]
	fn test_len_8() {
		let tree: AANode<i32> = tree_from([1, 2, 3, 4, 5, 6, 7, 8]);
		let expected =
			tree!(4 => [3, (2 => [2, 1, 3]), (6 => [2, 5, (7 => [1, Nil, 8])])]);
		assert_eq!(tree, expected);
	}

	#[test]
	fn test_len_9() {
		let tree: AANode<i32> = tree_from([1, 2, 3, 4, 5, 6, 7, 8, 9]);
		let expected = tree!(5 => [3, (2 => [2, 1, (3 =>  [1, Nil, 4])]), (7 => [2, 6, (8 => [1, Nil, 9])])]);
		assert_eq!(tree, expected);
	}

	#[test]
	fn test_len_10() {
		let tree: AANode<i32> = tree_from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
		let expected = tree!(5 => [3, (2 => [2, 1, (3 =>  [1, Nil, 4])]), (8 => [2, (6 => [1, Nil, 7]), (9 => [1, Nil, 10])])]);
		assert_eq!(tree, expected);
	}
}
