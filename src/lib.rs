
pub use visita_macros::{node_group, visitor};

pub trait NodeGroup<V> : Sized where V : VisitorGroup<Self> {
	type Data;

	fn accept(&self, v: &mut V) -> V::Output;
}

pub trait Node<V> : Sized where V : VisitorGroup<Self::Group> + Visitor<Self> {
	type Group : NodeGroup<V>;

	fn accept(&self, v: &mut V, data: &Data<V, Self>) -> <V as VisitorGroup<Self::Group>>::Output {
		v.visit(self, data)
	}
}

pub trait VisitorGroup<G> : Sized where G : NodeGroup<Self> {
	type Output;

	fn visit_group(&mut self, g: &G) -> Self::Output {
		g.accept(self)
	}
}

pub trait Visitor<N> : VisitorGroup<N::Group> where N : Node<Self> {
	fn visit(&mut self, node: &N, data: &Data<Self, N>) -> Self::Output;
}

pub type Data<V, N> = <<N as Node<V>>::Group as NodeGroup<V>>::Data;
