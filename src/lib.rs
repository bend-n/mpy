#![feature(if_let_guard, proc_macro_quote)]
mod run;
use std::ffi::CString;

use proc_macro::TokenStream;
use syn::Lit;
#[proc_macro]
pub fn eval(i: TokenStream) -> TokenStream {
    run::exec(&CString::new(i.to_string()).unwrap()).into()
}
