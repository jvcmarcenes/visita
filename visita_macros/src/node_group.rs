
use proc_macro2::{Ident, Span};
use quote::{quote, format_ident};
use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemEnum, Variant, Fields, Type, parse::Parse, Token, Visibility, VisPublic, VisRestricted, PathSegment, PathArguments, token::Paren, Path, punctuated::Punctuated};

struct Data(Type);

impl Parse for Data {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		let data = input.parse::<Ident>()?;
		if data != "data" { panic!("expected 'data'"); }
		input.parse::<Token![=]>()?;
		let ty = input.parse::<Type>()?;
		Ok(Data(ty))
	}
}

pub(crate) fn node_group(attr: TokenStream, item: TokenStream) -> TokenStream {
	let data = parse_macro_input!(attr as Data).0;

	let ItemEnum { attrs, vis, ident, mut variants, .. } = parse_macro_input!(item as ItemEnum);

	let node_ident = format_ident!("{ident}Node");

	let variant_idents = variants.iter().map(|v| v.ident.clone()).collect::<Vec<_>>();

	let visitor_bounds = quote! {
		visita::VisitorGroup<#ident> + #(visita::Visitor<#variant_idents>)+*
	};

	let base_vis = match vis.clone() {
		vis @ (Visibility::Public(_) | Visibility::Crate(_)) => vis,
		Visibility::Restricted(mut vis_rest) => {
			if vis_rest.in_token.is_some() {
				vis_rest.path.segments.insert(0, PathSegment { ident: Ident::new("super", Span::call_site()), arguments: PathArguments::None });
			}
			Visibility::Restricted(vis_rest)
		},
		Visibility::Inherited => {
			let mut segments = Punctuated::new();
			segments.push(PathSegment { ident: Ident::new("super", Span::call_site()), arguments: PathArguments::None });
			let path = Box::new(Path { leading_colon: None, segments });
			Visibility::Restricted(VisRestricted {
				pub_token: <Token![pub]>::default(),
				paren_token: Paren::default(),
				in_token: Some(<Token![in]>::default()),
				path
			})
		},
	};

	let nodes = variants.iter_mut().map(|Variant { ident: var_ident, fields, attrs, .. }| {
		let semi = if matches!(fields, Fields::Named(_)) { quote![] } else { quote![;] };
		fields.iter_mut().for_each(|field| {
			field.vis = Visibility::Public(VisPublic { pub_token: <Token![pub]>::default() });
		});
		quote! {
			#(#attrs)*
			#base_vis struct #var_ident #fields #semi
			impl<V> visita::Node<V> for #var_ident where V : #visitor_bounds {
				type Group = #ident;
			}
			impl #var_ident {
				pub fn to_node(self, data: #data) -> #ident {
					#ident {
						node: Box::new(#node_ident::#var_ident(self)),
						data,
					}
				}
			}
		}
	}).collect::<Vec<_>>();

	let mod_name = format_ident!("{}", ident.to_string().to_lowercase());

	let output = quote! {

		mod #mod_name {
			use super::*;

			#(#nodes)*

			#(#attrs)*
			enum #node_ident {
				#(#variant_idents(#variant_idents),)*
			}

			#(#attrs)*
			#base_vis struct #ident {
				node: Box<#node_ident>,
				data: #data,
			}

			impl<V> visita::NodeGroup<V> for #ident where V : #visitor_bounds {
				type Data = #data;
				fn accept(&self, v: &mut V) -> <V as visita::VisitorGroup<Self>>::Output {
					match self.node.as_ref() {
						#(#node_ident::#variant_idents(node) => visita::Node::<V>::accept(node, v, &self.data),)*
					}
				}
			}
		}

		#vis use #mod_name::{#ident, #(#variant_idents),*};

	};

	TokenStream::from(output)
}
