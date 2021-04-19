#![allow(clippy::never_loop)]
extern crate proc_macro;

mod util;
mod ast;
mod generator;

use proc_macro::TokenStream;

/// This turns regular rust function into N-API compatible native module
///
/// For example; given rust following here
/// ```ignore
/// fn sum(first: i32, second: i32) -> i32 {
///      return first+second
/// }
/// ```
///
/// into N-API module
/// ```ignore
/// #[no_mangle]
/// pub extern "C" fn n_sum(env: napi_env, cb_info: napi_callback_info) -> napi_value {
///     fn sum(first: i32, second: i32) -> i32 {
///       return first+second
///     }
///     let js_env = JsEnv::new(env);
///     let js_cb = result_to_napi!(js_env.get_cb_info(cb_info, 2),&js_env);
///     let first = result_to_napi!(js_cb.get_value::<i32>(0),&js_env);
///     let second = result_to_napi!(js_cb.get_value::<i32>(0),&js_env);
///     sum(msg).into_js(&js_env)
/// }
/// ```
#[proc_macro_attribute]
pub fn node_bindgen(args: TokenStream, item: TokenStream) -> TokenStream {
    use syn::AttributeArgs;

    use ast::FunctionAttributes;
    use ast::NodeItem;
    use generator::generate_function;
    use generator::generate_class;
    use generator::generate_datatype;

    let attribute_args = syn::parse_macro_input!(args as AttributeArgs);

    let attribute: FunctionAttributes = match FunctionAttributes::from_ast(attribute_args) {
        Ok(attr) => attr,
        Err(err) => return err.to_compile_error().into(),
    };

    let parsed_item = syn::parse_macro_input!(item as NodeItem);

    let out_express = match parsed_item {
        NodeItem::Function(fn_item) => generate_function(fn_item, attribute),
        NodeItem::Impl(impl_item) => generate_class(impl_item),
        NodeItem::Derive(struct_item) => generate_datatype(struct_item),
    };

    // used for debugging, if error occurs println do not work so should uncomment express
    // println!("{}", out_express);
    //let out_express = quote::quote! {};

    out_express.into()
}
