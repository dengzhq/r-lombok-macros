//! Macros for [`r-lombok`].
//!
//! [`r-lombok-macros`]: https://crates.io/crates/r-lombok-macros
//! # Example
//! ```
//! use std::fmt::Debug;
//! use r_lombok_macros::{Getter, Setter};
//! #[derive(Debug, Getter, Setter)]
//! struct Company {
//!     name: &'static str,
//!     boss: &'static str,
//!     number: u32,
//!     department: Vec<String>,
//! }
//!
//! #[derive(Getter, Setter)]
//! struct CompanyGen<T> where T: Debug {
//!     name: T,
//!     boss: &'static str,
//!     number: u32,
//!     department: Vec<String>,
//! }
//!
//! #[derive(Getter, Setter)]
//! struct CompanyWrap {
//!     sub_company: CompanyGen<Company>,
//!     name: &'static str,
//! }
//!
//! // Unit struct
//! #[derive(Getter, Setter, Debug)]
//! struct UnitStruct {}
//!
//! fn test_getter_setter() {
//!     let mut xp = Company {
//!         name: "xp",
//!         boss: "Big Brother",
//!         number: u32::MAX,
//!         department: vec!["HR".to_owned(), "Finance".to_owned()],
//!     };
//!     println!("xp name = {:?} boss = {:?} number = {:?} department = {:?}", xp.get_name(), xp.get_boss(), xp.get_number(), xp.get_department());
//!     xp.set_name("set_name");
//!     xp.set_boss("set_boss");
//!     xp.set_number(u32::MIN);
//!     xp.set_department(vec!["department".to_owned()]);
//!
//!
//!     let xp_t = CompanyGen::<Company> {
//!         name: xp,
//!         boss: "Big Brother",
//!         number: u32::MAX,
//!         department: vec!["HR".to_owned(), "Finance".to_owned()],
//!     };
//!     println!("xp_t name = {:?} boss = {:?} number = {:?} department = {:?}", xp_t.get_name(), xp_t.get_boss(), xp_t.get_number(), xp_t.get_department());
//!
//!     let xp_wrap = CompanyWrap {
//!         sub_company: xp_t,
//!         name: "xp_wrap",
//!     };
//!     println!("xp_wrap name = {:?} sub_company = {:?}", xp_wrap.get_name(), xp_wrap.get_sub_company().get_name());
//! }
//! ```

#![warn(
clippy::all,
clippy::dbg_macro,
clippy::todo,
clippy::empty_enum,
clippy::enum_glob_use,
clippy::mem_forget,
clippy::unused_self,
clippy::filter_map_next,
clippy::needless_continue,
clippy::needless_borrow,
clippy::match_wildcard_for_single_variants,
clippy::if_let_mutex,
clippy::mismatched_target_os,
clippy::await_holding_lock,
clippy::match_on_vec_items,
clippy::imprecise_flops,
clippy::suboptimal_flops,
clippy::lossy_float_literal,
clippy::rest_pat_in_fully_bound_structs,
clippy::fn_params_excessive_bools,
clippy::exit,
clippy::inefficient_to_string,
clippy::linkedlist,
clippy::macro_use_imports,
clippy::option_option,
clippy::verbose_file_reads,
clippy::unnested_or_patterns,
clippy::str_to_string,
future_incompatible,
nonstandard_style,
missing_debug_implementations,
missing_docs
)]
#![forbid(unsafe_code)]

use proc_macro::TokenStream;

use quote::{quote, ToTokens};
use syn::parse::Parse;

mod getter;
mod setter;
mod data;
mod accessors;
mod to_string;

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

/// like java lombok Data annotation, but only contains getter、setter method
///
///
/// ```
/// use r_lombok_macros::Data;
/// #[derive(Data)]
/// struct Company {
///     name: &'static str,
///     boss: &'static str,
///     number: u32,
///     department: Vec<String>,
/// }
///
/// fn test_data() {
///     let mut xp = Company {
///         name: "xp",
///         boss: "Big Brother",
///         number: u32::MAX,
///         department: vec!["HR".to_owned(), "Finance".to_owned()],
///     };
///     println!("xp name = {:?} boss = {:?} number = {:?} department = {:?}", xp.get_name(), xp.get_boss(), xp.get_number(), xp.get_department());
///     xp.set_name("set_name");
///     xp.set_boss("set_boss");
///     xp.set_number(u32::MIN);
///     xp.set_department(vec!["XP-HR".to_owned(), "XP-Finance".to_owned()]);
///     println!("xp data name = {:?} boss = {:?} number = {:?} department = {:?}", xp.get_name(), xp.get_boss(), xp.get_number(), xp.get_department());
/// }
/// ```
///
#[proc_macro_derive(Data)]
pub fn derive_data(input: TokenStream) -> TokenStream {
    expand_with(input, data::expand)
}

/// like java lombok ToString annotation
///
/// you can use #[derive(Debug)] instead,but ToString contains Debug and impl Display
/// ```
/// use r_lombok_macros::ToString;
/// #[derive(ToString)]
/// struct Company {
///     name: &'static str,
///     boss: &'static str,
///     number: u32,
///     department: Vec<String>,
/// }
///
/// fn test_to_string() {
///     let xp = Company {
///         name: "xp",
///         boss: "Big Brother",
///         number: u32::MAX,
///         department: vec!["HR".to_owned(), "Finance".to_owned()],
///     };
///     println!("xp = {}", xp.to_string());
///     println!("xp = {:?}", xp.to_string());
/// }
/// ```
///
#[proc_macro_derive(ToString)]
pub fn derive_to_string(input: TokenStream) -> TokenStream {
    expand_with(input, to_string::expand)
}

/// like java lombok accessors annotation,but contains data and no chain attr
///
///```
/// use r_lombok_macros::Accessors;
///#[derive(Accessors)]
/// struct Company {
///     name: &'static str,
///     boss: &'static str,
///     number: u32,
///     department: Vec<String>,
/// }
///
/// fn test_accessors() {
///     let mut xp = Company {
///         name: "xp",
///         boss: "Big Brother",
///         number: u32::MAX,
///         department: vec!["HR".to_owned(), "Finance".to_owned()],
///     };
///     println!("xp name = {:?} boss = {:?} number = {:?} department = {:?}", xp.get_name(), xp.get_boss(), xp.get_number(), xp.get_department());
///     xp.set_name("set_name").set_boss("set_boss").set_number(u32::MIN).set_department(vec!["XP-HR".to_owned(), "XP-Finance".to_owned()]);
///     println!("xp accessors name = {:?} boss = {:?} number = {:?} department = {:?}", xp.get_name(), xp.get_boss(), xp.get_number(), xp.get_department());
/// }
/// ```
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