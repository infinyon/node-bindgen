use proc_macro2::TokenStream;
use quote::quote;

use crate::ast::FunctionArgType;
use crate::ast::Method;
use crate::ast::Class;
use crate::ast::FunctionArgs;
use crate::util::arg_ident;
use crate::util::ident;

pub fn generate_class_arg(method: Option<&Method>, class: &Class) -> TokenStream {
    if let Some(method) = method {
        let class_name = class.my_type().ident().unwrap(); // class should have identifier
        let args = generate_args(&method.args);
        let struct_args = generate_structure_args(&method.args);

        let constr_conversion = as_constructor_try_to_js(&method.args);
        let invocation = as_constructor_invocation(&method.args);
        let construct_name = ident(&format!("{}Constructor", class_name));
        quote! {

            pub struct #construct_name {
                #args
            }

            impl #construct_name {
                pub fn new(#args) -> Self {
                    Self {
                        #struct_args
                    }
                }
            }

            impl node_bindgen::core::TryIntoJs for #construct_name {

                fn try_to_js(self, js_env: &node_bindgen::core::val::JsEnv) -> Result<node_bindgen::sys::napi_value,node_bindgen::core::NjError> {

                    #constr_conversion
                    let new_instance = #class_name::new_instance(js_env,vec![#invocation])?;
                    Ok(new_instance)
                }
            }
        }
    } else {
        quote! {}
    }
}

/// generate arg with type, for used in defining structures
fn generate_args(args: &FunctionArgs) -> TokenStream {
    let args = args
        .args
        .iter()
        //  .filter(|arg| !arg.is_callback())
        .enumerate()
        .map(|(i, arg)| {
            let name = arg_ident(i);
            match &arg.typ {
                FunctionArgType::Path(ty) => {
                    let inner = ty.expansion();
                    quote! {
                        #name: #inner
                    }
                }
                FunctionArgType::Closure(_) => {
                    quote! { compile_error!("closure can't be used in constructor ")}
                }
                //      FunctionArgType::JSCallback(_) => quote! { compile_error!("JsCallback can't be used in constructor")},
                FunctionArgType::Ref(_) => {
                    quote! { compile_error!("ref can't be used in constructor")}
                }
            }
        });

    quote! {
        #(#args),*
    }
}

fn generate_structure_args(args: &FunctionArgs) -> TokenStream {
    let args = args
        .args
        .iter()
        //   .filter(|arg| !arg.is_callback())
        .enumerate()
        .map(|(i, _arg)| {
            let name = arg_ident(i);
            quote! {
                #name
            }
        });

    quote! {
        #(#args),*
    }
}

/// generate expression to convert constructor into new instance
fn as_constructor_try_to_js(args: &FunctionArgs) -> TokenStream {
    let arg_len = args.len();
    let args = (0..arg_len).map(|i| {
        let arg_name = arg_ident(i);
        quote! {
            let #arg_name = self.#arg_name.try_to_js(js_env)?;
        }
    });

    quote! {

        #(#args)*

    }
}

fn as_constructor_invocation(args: &FunctionArgs) -> TokenStream {
    let arg_len = args.len();
    let args = (0..arg_len).map(|i| {
        let arg_name = arg_ident(i);
        quote! {
            #arg_name
        }
    });

    quote! {

        #(#args),*

    }
}
