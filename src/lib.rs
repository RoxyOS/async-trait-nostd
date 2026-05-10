//! `async-trait` with `no_std` support.
//!
//! ## `Std` usage
//!
//! `async-trait-nostd` doesnt work under `std` environment, use the original `async-trait` instead.
//!
//! ---
//!
//! For more information, refer to the original [docs](https://docs.rs/async-trait/latest/async_trait/)
#![allow(
    clippy::default_trait_access,
    clippy::doc_markdown,
    clippy::elidable_lifetime_names,
    clippy::expl_impl_clone_on_copy, // https://github.com/rust-lang/rust-clippy/issues/15842
    clippy::explicit_auto_deref,
    clippy::if_not_else,
    clippy::items_after_statements,
    clippy::match_like_matches_macro,
    clippy::module_name_repetitions,
    clippy::needless_lifetimes,
    clippy::shadow_unrelated,
    clippy::similar_names,
    clippy::too_many_lines,
    clippy::trivially_copy_pass_by_ref
)]

extern crate proc_macro;

mod args;
mod bound;
mod expand;
mod lifetime;
mod parse;
mod receiver;
mod verbatim;

use crate::args::Args;
use crate::expand::expand;
use crate::parse::Item;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn async_trait(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as Args);
    let mut item = parse_macro_input!(input as Item);
    expand(&mut item, args.local);
    TokenStream::from(quote!(#item))
}
