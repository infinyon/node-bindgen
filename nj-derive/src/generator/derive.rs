use quote::quote;
use proc_macro2::TokenStream;
use proc_macro2::Span;

use syn::DeriveInput;

pub fn generate_datatype(input_struct: DeriveInput) -> TokenStream {
    println!("In gen datatype");

    quote! {
        // We are in a datatype gen
        #input_struct
    }
}