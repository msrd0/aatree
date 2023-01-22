use crate::{AATreeMap, AATreeSet};
use openapi_type::{ObjectVisitor, OpenapiType, Visitor};

impl<T> OpenapiType for AATreeSet<T>
where
	T: OpenapiType
{
	fn visit_type<V: Visitor>(visitor: &mut V) {
		T::visit_type(visitor.visit_array(None, true))
	}
}

impl<K, T> OpenapiType for AATreeMap<K, T>
where
	T: OpenapiType
{
	fn visit_type<V: Visitor>(visitor: &mut V) {
		T::visit_type(visitor.visit_object().visit_additional())
	}
}
