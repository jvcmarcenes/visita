
use proc_macro::TokenStream;
use proc_macro2::Ident;
use syn::{Type, parse::Parse, Token, parse_macro_input, DeriveInput};
use quote::quote;

struct Visitor(Ident, Type);

impl Parse for Visitor {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		let node = input.parse::<Ident>()?;
		input.parse::<Token![,]>()?;
		let output = input.parse::<Ident>()?;
		if output != "output" { panic!("expected 'output'"); }
		input.parse::<Token![=]>()?;
		let ty = input.parse::<Type>()?;
		Ok(Visitor(node, ty))
	}
}

pub(crate) fn visitor(attr: TokenStream, item: TokenStream) -> TokenStream {
	let ref item @ DeriveInput { ref ident, ref generics, .. } = parse_macro_input!(item as DeriveInput);
	let Visitor(node, output) = parse_macro_input!(attr as Visitor);

	quote! {
		#item
		
		impl #generics visita::Visitor<#node> for #ident #generics {
			type Output = #output;
		}
	}.into()
}
