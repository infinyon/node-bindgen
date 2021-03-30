use quote::quote;
use proc_macro2::TokenStream;
use proc_macro2::Span;
use syn::ItemFn;
use syn::Ident;
use syn::LitInt;
use syn::LitStr;

use crate::ast::FunctionArgs;
use crate::ast::FunctionArg;
use crate::ast::FunctionAttributes;
use crate::ast::FunctionArgType;
use crate::util::ident;

use super::FnGeneratorCtx;
use super::generate_napi_code;
use super::generate_property_code;

use closure::generate_closure_invocation;

pub type CbArgs = Vec<TokenStream>;

pub use arg_extraction::*;
pub use args_input::*;

/// generate JS wrapper to translate rust function
pub fn generate_function(input_fn: ItemFn, attributes: FunctionAttributes) -> TokenStream {
    match FunctionArgs::from_ast(&input_fn.sig) {
        Err(err) => err.to_compile_error(),
        Ok(args) => {
            // validate additional attribute in method context

            if !args.is_method {
                if let Err(err) = attributes.valid_as_non_method() {
                    return err.to_compile_error();
                }
            }

            let ctx = FnGeneratorCtx::new(&input_fn.sig, &args, &attributes);

            if attributes.is_constructor() {
                return quote! {
                    #input_fn
                };
            }

            let napi_code = generate_napi_code(&ctx, &input_fn);
            let property_code = generate_property_code(&ctx);

            let expansion = quote! {

                #napi_code

                #property_code

            };

            expansion
        }
    }
}

/// generate code to extract Rust values from JS environment
/// Given rust code like this:
///
///     fn sum(first: i32, second: i32) -> i32 {
///         first + second
///     }
///
///
/// Generate extract code like as below:
///     let result: Result<node_bindgen::sys::napi_value, node_bindgen::core::NjError> =
///         (move || {
///             let js_cb = js_env.get_cb_info(cb_info, 2)?;
///             let rust_value_0 = js_cb.get_value::<i32>(0)?;
///             let rust_value_1 = js_cb.get_value::<i32>(1)?;
///             sum(rust_value_0, rust_value_1).try_to_js(&js_env)
///         })();
///     result.into_js(&js_env)
///
/// Code generation does
///   - compute number of parameters from input signatures
///   - for each arg type, generates converting line
///             let rust_value_{N} = js_cb.get_value::<{T}>(N)?;
///   - then invoke original rust code
///
/// This leverages TryIntoJs trait
///
pub fn generate_rust_invocation(ctx: &FnGeneratorCtx, cb_args: &mut CbArgs) -> TokenStream {
    // code to convert extract rust values from Js Env
    let js_to_rust_values = arg_extraction::as_arg_token(ctx);

    let rust_invoke = invocation::rust_invocation(ctx, cb_args);

    // if this is async, wrap with JsFuture
    let rust_invoke_ft_wrapper = if ctx.is_async() {
        let async_name = format!("{}_ft", ctx.fn_name());
        let async_lit = LitStr::new(&async_name, Span::call_site());
        quote! {
            (node_bindgen::core::JsPromiseFuture::new(
                #rust_invoke, #async_lit
            )).try_to_js(&js_env)
        }
    } else {
        quote! {
            #rust_invoke.try_to_js(&js_env)
        }
    };

    let receiver = if ctx.is_method() {
        quote! {
            let receiver = (js_cb.unwrap_mut::<Self>()?);
        }
    } else {
        quote! {}
    };

    quote! {

        let result: Result<node_bindgen::sys::napi_value,node_bindgen::core::NjError> = ( move || {

            #js_to_rust_values

            #receiver

            #rust_invoke_ft_wrapper

        })();


        result.into_js(&js_env)
    }
}

/// generate rust value from js cb context
/// ```ignore
/// let js_cb = js_env.get_cb_info(cb_info, 2)?;
/// let rust_value_0 = js_cb.get_value::<i32>(0)?;
/// let rust_value_1 = js_cb.get_value::<i32>(1)?;
/// ```
mod arg_extraction {

    use super::*;
    use crate::ast::ClosureType;

    pub fn as_arg_token(ctx: &FnGeneratorCtx) -> TokenStream {
        let count_literal = ctx.args.len().to_string();
        let js_count = LitInt::new(&count_literal, Span::call_site());

        //println!("fc arguments: {:#?}",ctx.args);

        let rust_args = ctx
            .args
            .inner()
            .iter()
            .enumerate()
            .map(|(i, arg)| js_to_rust_token_stream(i, arg, ctx));

        quote! {

            let mut js_cb = js_env.get_cb_info(cb_info, #js_count)?;

            #(#rust_args)*

        }
    }

    /// generate expression to extract rust value from Js env
    /// example as below:
    ///     let r_arg1 = cb.get_value::<f64>(1)?;
    ///
    fn js_to_rust_token_stream(
        arg_index: usize,
        arg: &FunctionArg,
        ctx: &FnGeneratorCtx,
    ) -> TokenStream {
        // println!("js to rust arg: {}, {:#?}",arg_index,arg.typ);
        match &arg.typ {
            FunctionArgType::Closure(ty) => {
                if ctx.is_async() || ctx.attributes.is_multi_threaded() {
                    generate_as_async_token_stream(ty, arg_index, ctx)
                } else {
                    rust_value(ctx.callback_type_name(), arg_index)
                }
            }
            FunctionArgType::Path(ty) => rust_value(ty.expansion(), arg_index),
            FunctionArgType::Ref(ty) => rust_value(ty.expansion(), arg_index),
            FunctionArgType::Tuple(ty) => rust_value(ty.expansion(), arg_index),
        }
    }

    /// generate thread safe function when callback are used in the async
    /// for example:
    ///     let r_arg1 = cb.create_thread_safe_function("hello_sf",0,Some(hello_callback_js))?;
    fn generate_as_async_token_stream(
        ty: &ClosureType,
        index: usize,
        ctx: &FnGeneratorCtx,
    ) -> TokenStream {
        let sf_identifier = LitStr::new(&format!("{}_sf", ctx.fn_name()), Span::call_site());
        let rust_var_name = rust_arg_var(index);
        let js_cb_completion = ty.async_js_callback_identifier();
        let arg_index = LitInt::new(&index.to_string(), Span::call_site());
        quote! {
            let #rust_var_name = js_cb.create_thread_safe_function_at(#sf_identifier,#arg_index,Some(#js_cb_completion))?;
        }
    }
}

/// generating code to invoke rust function
mod invocation {

    use super::*;

    /// generate code to invoke rust function inside js wrapper
    /// for example
    ///         sum(rust_value_0, rust_value_1).try_to_js(&js_env)
    ///
    pub fn rust_invocation(ctx: &FnGeneratorCtx, cb_args: &mut CbArgs) -> TokenStream {
        let rust_args_input: Vec<TokenStream> = rust_args_input(ctx, cb_args);
        let rust_fn_ident = ctx.fn_name();

        if ctx.is_method() {
            quote! {
                receiver.#rust_fn_ident( #(#rust_args_input),* )
            }
        } else {
            quote! {
                #rust_fn_ident( #(#rust_args_input),* )
            }
        }
    }
}

mod closure {

    use super::*;

    use crate::ast::ClosureType;

    /// for closure, we need create wrapper closure that translates inner closure
    /// for example, if we have following rust closure
    /// ```ignore
    /// fn rust_fn<F: Fn(i32)>(cb: F)
    /// ```
    ///
    /// we need to invoke in the bindgen code as below
    /// ```ignore
    /// let js_cb = js_env.get_cb_info(cb_info,1)?;
    /// let rust_value_0 = js_cb.get_value::<JsCallbackFunction>(0)?;
    ///
    ///     rust_fn( move |cb_arg0: i32| {
    ///         let result = (|| {
    ///             let args = vec![cb_arg0];
    ///             rust_value_0.call(args)
    ///         })();
    ///         result.into_js(&js_env)
    ///     }).try_to_js(&js_env)
    /// ```
    ///
    pub fn generate_closure_invocation(
        closure: &ClosureType,
        arg_index: usize,
        closure_var: &Ident,
        ctx: &FnGeneratorCtx,
        cb_args: &mut CbArgs,
    ) -> TokenStream {
        let args: Vec<TokenStream> = closure
            .inputs
            .iter()
            .enumerate()
            .map(|(index, ty)| {
                let arg_name = format!("cb_arg{}", index);
                let var_name = ident(&arg_name);
                let type_name = ty.expansion();
                quote! {
                    #var_name: #type_name
                }
            })
            .collect();

        let inner_closure = if ctx.is_async() || ctx.attributes().is_multi_threaded() {
            as_async_arg_token_stream(closure, closure_var, ctx, cb_args)
        } else {
            as_sync_arg_token_stream(closure, arg_index, closure_var)
        };

        quote! {
           move | #(#args),* | {

               #inner_closure
           }
        }
    }

    /// generate as argument to sync rust function or method
    /// since this is closure, we generate closure
    fn as_sync_arg_token_stream(
        closure: &ClosureType,
        _i: usize,
        closure_var: &Ident,
    ) -> TokenStream {
        let js_conversions: Vec<TokenStream> = closure
            .inputs
            .iter()
            .enumerate()
            .map(|(index, _path)| {
                let arg_name = format!("cb_arg{}", index);
                let var_name = Ident::new(&arg_name, Span::call_site());
                quote! {
                    #var_name
                }
            })
            .collect();

        quote! {

            // invoke sync closure
            let result = (|| {
                let args = vec![
                    #(#js_conversions),*
                ];
                #closure_var.call(args)
            })();

            result.into_js(&js_env);

        }
    }

    fn as_async_arg_token_stream(
        closure: &ClosureType,
        closure_var: &Ident,
        _ctx: &FnGeneratorCtx,
        cb_args: &mut CbArgs,
    ) -> TokenStream {
        let arg_struct_name = Ident::new(&format!("Arg{}", closure.ident), Span::call_site());
        let arg_cb_complete = closure.async_js_callback_identifier();
        let struct_fields: Vec<TokenStream> = closure
            .inputs
            .iter()
            .enumerate()
            .map(|(index, ty)| {
                let var_name = Ident::new(&format!("arg{}", index), Span::call_site());
                let type_name = ty.expansion();
                quote! {
                    #var_name: #type_name
                }
            })
            .collect();

        let js_complete_conversions: Vec<TokenStream> = closure
            .inputs
            .iter()
            .enumerate()
            .map(|(index, _path)| {
                let js_var_iden = Ident::new(&format!("js_arg{}", index), Span::call_site());
                let arg_idn = Ident::new(&format!("arg{}", index), Span::call_site());
                quote! {
                    let #js_var_iden = my_val.#arg_idn.try_to_js(&js_env)?;
                }
            })
            .collect();

        let js_call: Vec<TokenStream> = closure
            .inputs
            .iter()
            .enumerate()
            .map(|(index, _path)| {
                let js_var_iden = Ident::new(&format!("js_arg{}", index), Span::call_site());
                quote! {
                    #js_var_iden
                }
            })
            .collect();

        cb_args.push(quote!{

            #[derive(Debug)]
            struct #arg_struct_name {
                #(#struct_fields),*
            }

            extern "C" fn #arg_cb_complete(
                env: node_bindgen::sys::napi_env,
                js_cb: node_bindgen::sys::napi_value,
                _context: *mut ::std::os::raw::c_void,
                data: *mut ::std::os::raw::c_void) {

                if env != std::ptr::null_mut() {

                    node_bindgen::core::log::debug!("async cb invoked");
                    let js_env = node_bindgen::core::val::JsEnv::new(env);
                    let result: Result<(), node_bindgen::core::NjError> = (move ||{
                        let global = js_env.get_global()?;
                        let my_val: Box<#arg_struct_name> = unsafe { Box::from_raw(data as *mut #arg_struct_name) };
                        node_bindgen::core::log::trace!("arg: {:#?}",my_val);
                        #(#js_complete_conversions)*

                        node_bindgen::core::log::debug!("async cb, invoking js cb");
                        js_env.call_function(global,js_cb,vec![#(#js_call),*])?;
                        node_bindgen::core::log::trace!("async cb, done");
                        Ok(())
                    })();

                    node_bindgen::core::assert_napi!(result)
                }
            }

        });

        let args: Vec<TokenStream> = closure
            .inputs
            .iter()
            .enumerate()
            .map(|(index, _path)| {
                let arg_name = Ident::new(&format!("arg{}", index), Span::call_site());
                let cb_name: Ident = Ident::new(&format!("cb_arg{}", index), Span::call_site());
                quote! {
                    #arg_name: #cb_name
                }
            })
            .collect();

        quote! {

            let arg = #arg_struct_name {
                #(#args),*
            };

            node_bindgen::core::log::trace!("converting rust to raw ptr");
            let my_box = Box::new(arg);
            let ptr = Box::into_raw(my_box);

            #closure_var.call(Some(ptr as *mut core::ffi::c_void)).expect("callback should work");

        }
    }
}

/// generate expression to convert napi value to rust value from callback
/// ```ignore
/// let rust_value_0 = js_cb.get_value_at::<&[u8]>(0)?;
/// ```
fn rust_value(type_name: TokenStream, index: usize) -> TokenStream {
    let arg_index = LitInt::new(&index.to_string(), Span::call_site());
    let rust_value = rust_arg_var(index);
    quote! {
        let #rust_value = js_cb.get_value_at::<#type_name>(#arg_index)?;
    }
}

/// generate rust_value with index
fn generate_rust_arg_var(index: usize) -> TokenStream {
    let var_name = rust_arg_var(index);

    quote! {
        #var_name
    }
}

/// given index, generate rust_value var name
fn rust_arg_var(index: usize) -> Ident {
    let var_name = format!("rust_value_{}", index);
    Ident::new(&var_name, Span::call_site())
}

mod args_input {

    use super::*;

    pub fn rust_args_input(ctx: &FnGeneratorCtx, cb_args: &mut CbArgs) -> Vec<TokenStream> {
        let mut arg_index = 0;
        ctx.args
            .args
            .iter()
            .map(|arg| as_arg_token_stream(&mut arg_index, arg, ctx, cb_args))
            .collect()
    }

    // generate code as part of invoking rust function
    /// so given rust function rust_fn
    /// it will pass parameter like rust_fn(rust_value_0,rust_value_1,...)
    /// only special case is closure where we pass closure
    fn as_arg_token_stream(
        index: &mut usize,
        arg: &FunctionArg,
        ctx: &FnGeneratorCtx,
        cb_args: &mut CbArgs,
    ) -> TokenStream {
        match &arg.typ {
            FunctionArgType::Closure(t) => {
                let arg_name = rust_arg_var(*index);
                let output = generate_closure_invocation(t, *index, &arg_name, ctx, cb_args);
                //println!("closure: {}",output);
                *index += 1;
                output
            }
            _ => {
                let output = generate_rust_arg_var(*index);
                *index += 1;
                output
            }
        }
    }
}
