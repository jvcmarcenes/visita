
use proc_macro::TokenStream;

mod node_group;
mod visitor;

#[proc_macro_attribute]
pub fn node_group(attr: TokenStream, item: TokenStream) -> TokenStream {
	node_group::node_group(attr, item)
}

#[proc_macro_attribute]
pub fn visitor(attr: TokenStream, item: TokenStream) -> TokenStream {
	visitor::visitor(attr, item)
}
