use proc_macro2::TokenStream;
use quote::quote;

use crate::generator::FnGeneratorCtx;
use crate::ast::Method;

/// generate class constructor 
pub fn class_constructor(method: Option<&Method>) -> TokenStream {

    use crate::generator::as_arg_token;
    use crate::generator::rust_args_input;

    
    let expansion = if let Some(method) = method  {


        let method_ident = &method.method_name();
    
    
        let ctx = FnGeneratorCtx::new(&method.method.sig,&method.args,&method.attributes);
        let arg_tokens = as_arg_token(&ctx);
        
        let mut cb_args = vec![];
        let rust_inputs = rust_args_input(&ctx,&mut cb_args);
        
        quote! {

            #arg_tokens

            let rust_value = Self::#method_ident(  #(#rust_inputs),* );
            Ok((rust_value,js_cb))
            
        }
    } else {
        quote! {
            let js_cb = js_env.get_cb_info(cb_info,0)?;
            Ok((Self::new(),js_cb))
        }
    };
    
    quote! {
        fn create_from_js(
            js_env: &node_bindgen::core::val::JsEnv, 
            cb_info: node_bindgen::sys::napi_callback_info) -> 
         Result<(Self,node_bindgen::core::val::JsCallback),node_bindgen::core::NjError> {

            #expansion
        }
    }
}
