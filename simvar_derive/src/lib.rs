extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(SimVarStruct)]
pub fn simvar_struct_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    let expanded = quote! {
        impl SimVarStruct for #name {
            fn from_raw(data: *const u8) -> Self {
                unsafe { std::ptr::read_unaligned(data as *const #name) }
            }
        }
    };
    TokenStream::from(expanded)
}
