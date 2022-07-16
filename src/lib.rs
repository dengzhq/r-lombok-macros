//! Macros for [`r-lombok`].
//!
//! [`r-lombok-macros`]: https://crates.io/crates/r-lombok-macros
extern crate proc_macro;

use proc_macro::TokenStream;

use quote::{quote, ToTokens};
use syn::parse::Parse;

mod getter;
mod setter;

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