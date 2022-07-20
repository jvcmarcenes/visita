
pub use visita_macros::{node_group, visitor};

// responsible for routing the visit methods to the different nodes
pub trait NodeFamily<V> : Sized where V : Visitor<Self> {
	type Data;

	fn accept(&self, v: &mut V) -> V::Output;
}

// responsible for associating a node to a collection of nodes
pub trait Node<V> : Sized where V : Visitor<Self::Family> + Visit<Self> {
	type Family : NodeFamily<V>;

	fn accept(&self, v: &mut V, data: &Data<V, Self>) -> V::Output {
		v.visit(self, data)
	}
}

// responsible for dictating the output of traversing a group of nodes
pub trait Visitor<F> : Sized where F : NodeFamily<Self> {
	type Output;
}

/// Implements the visiting logic for a node
pub trait Visit<N> : Visitor<N::Family> where N : Node<Self> {
	fn visit(&mut self, node: &N, data: &Data<Self, N>) -> Self::Output;
}

/// Shorthand for getting the data type from a node, as it can get quite verbose
pub type Data<V, N> = <<N as Node<V>>::Family as NodeFamily<V>>::Data;
