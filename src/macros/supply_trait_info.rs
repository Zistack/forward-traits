use syn::{Visibility, Path, Token, parse};
use syn::parse::{Result, Error};
use syn_derive::Parse;
use quote::quote;

use crate::uncurry::{get_macro_ident, get_path_ident, gen_uncurry_macro};
use crate::syn::trait_def_info::TraitDefInfo;

#[derive (Parse)]
struct SupplyForwardingInfoForTrait
{
	forwarded_trait: Path,
	_comma: Token! [,],
	visibility: Visibility,
	trait_def_info: TraitDefInfo
}

fn try_supply_forwarding_info_for_trait_impl (input: proc_macro::TokenStream)
-> Result <proc_macro2::TokenStream>
{
	let SupplyForwardingInfoForTrait
	{
		forwarded_trait,
		visibility,
		trait_def_info,
		..
	}
		= parse (input)?;

	let use_statement = quote! (#visibility use #forwarded_trait;);

	let uncurry_macro = gen_uncurry_macro
	(
		visibility,
		get_macro_ident (&get_path_ident (&forwarded_trait)?),
		trait_def_info
	);

	Ok (quote! (#use_statement #uncurry_macro))
}

pub fn supply_forwarding_info_for_trait_impl (input: proc_macro::TokenStream)
-> proc_macro::TokenStream
{
	try_supply_forwarding_info_for_trait_impl (input)
		. unwrap_or_else (Error::into_compile_error)
		. into ()
}
