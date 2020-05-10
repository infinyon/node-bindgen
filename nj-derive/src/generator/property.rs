use quote::quote;
use proc_macro2::TokenStream;

use crate::util::ident;
use super::FnGeneratorCtx;


/// generate code to register this function property to global property
pub fn generate_property_code(ctx: &FnGeneratorCtx) -> TokenStream {


    if ctx.is_method() {
        return quote! {};
    }

    let ident_n_api_fn = ctx.napi_fn_id();
    let ident_register_fn = ident(&format!("register_{}", ident_n_api_fn));
    let property_name_literal = ctx.property_name();
  
    quote! {
        #[node_bindgen::core::ctor]
        fn #ident_register_fn() {

            let property = node_bindgen::core::Property::new(#property_name_literal).method(#ident_n_api_fn);
            node_bindgen::core::submit_property(property);
        }

    }
}
