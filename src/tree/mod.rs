//mod insert;
mod node;
//mod remove;
//mod traverse;

use node::AANode;
//pub use traverse::TraverseStep;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AATree<T> {
	root: Option<AANode<T>>
}
