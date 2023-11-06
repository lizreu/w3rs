extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro]
pub fn cc(input: TokenStream) -> TokenStream {
    w3_derive_impl::cc(input.into()).into()
}
