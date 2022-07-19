//! Macros for [`r-lombok`].
//!
//! [`r-lombok-macros`]: https://crates.io/crates/r-lombok-macros
extern crate proc_macro;

use proc_macro::TokenStream;

use quote::{quote, ToTokens};
use syn::parse::Parse;

mod getter;
mod setter;
mod data;
mod accessors;

/// field get method prefix
pub(crate) const METHOD_GET_PREFIX: &str = "get_";
/// field set method prefix
pub(crate) const METHOD_SET_PREFIX: &str = "set_";


/// like java lombok Getter annotation
///
///
#[proc_macro_derive(Getter)]
pub fn derive_getter(input: TokenStream) -> TokenStream {
    expand_with(input, getter::expand)
}

/// like java lombok Setter annotation
///
///
#[proc_macro_derive(Setter)]
pub fn derive_setter(input: TokenStream) -> TokenStream {
    expand_with(input, setter::expand)
}

/// like java lombok Data annotation, but only contains getter、setter
///
///
#[proc_macro_derive(Data)]
pub fn derive_data(input: TokenStream) -> TokenStream {
    expand_with(input, data::expand)
}


/// like java lombok accessors annotation,but contains data and no chain attr
///
///
#[proc_macro_derive(Accessors)]
pub fn derive_accessors(input: TokenStream) -> TokenStream {
    expand_with(input, accessors::expand)
}

fn expand_with<F, I, K>(input: TokenStream, f: F) -> TokenStream
    where
        F: FnOnce(I) -> syn::Result<K>,
        I: Parse,
        K: ToTokens,
{
    let r = syn::parse(input);
    expand(r.and_then(f))
}

fn expand<T>(result: syn::Result<T>) -> TokenStream
    where
        T: ToTokens,
{
    match result {
        Ok(tokens) => {
            let tokens = (quote! { #tokens }).into();
            if std::env::var_os("R_LOMBOK_MACROS_DEBUG").is_some() {
                eprintln!("{}", tokens);
            }
            tokens
        }
        Err(err) => err.into_compile_error().into(),
    }
}