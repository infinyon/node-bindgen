mod constructor;
mod arg;

use quote::quote;
use proc_macro2::TokenStream;
use syn::ItemImpl;

use crate::ast::Class;
use crate::util::ident;
use crate::util::lit_str;

pub fn generate_class(impl_item: ItemImpl) -> TokenStream {
    match Class::from_ast(&impl_item) {
        Err(err) => err.to_compile_error(),
        Ok(class) => {
            let class_helper = generate_class_helper(class);

            quote! {

                #impl_item

                #class_helper

            }
        }
    }
}

// generate internal module that contains Js class helper
fn generate_class_helper(class: Class) -> TokenStream {
    use constructor::class_constructor;
    use arg::generate_class_arg;

    let constructor_method = class.constructor();
    let type_name = class.self_ty.ident().unwrap();
    let lifetime = class.self_ty.lifetime();
    let impl_for_block = if let Some(lifetime) = lifetime {
        quote! {
            #type_name<#lifetime>
        }
    } else {
        quote! {
            #type_name
        }
    };

    let helper_module_name = ident(&format!("{type_name}_helper").to_lowercase());

    let class_type_lit = lit_str(&format!("{type_name}"));
    let properties = generate_properties(&class);
    let constructor_exp = class_constructor(constructor_method);
    let class_arg_exp = generate_class_arg(constructor_method, &class);
    let construct_name = ident(&format!("{type_name}Constructor"));

    quote! {

        use #helper_module_name::#construct_name;

        mod #helper_module_name {

            use std::ptr;
            use node_bindgen::core::JSClass;

            use super::#type_name;

            static mut CLASS_CONSTRUCTOR: node_bindgen::sys::napi_ref = ptr::null_mut();

            impl node_bindgen::core::JSClass for #impl_for_block {
                const CLASS_NAME: &'static str = #class_type_lit;

                fn set_constructor(constructor: node_bindgen::sys::napi_ref) {
                    node_bindgen::core::log::trace!("set constructor");
                    unsafe {
                        CLASS_CONSTRUCTOR = constructor;
                    }
                }

                fn get_constructor() -> node_bindgen::sys::napi_ref {
                    unsafe { CLASS_CONSTRUCTOR }
                }

                fn properties() -> node_bindgen::core::PropertiesBuilder {

                    vec![
                        #(#properties),*
                    ].into()
                }

                #constructor_exp

            }

            #class_arg_exp

            use node_bindgen::core::submit_register_callback;


            #[node_bindgen::core::ctor]
            fn register_class() {
                node_bindgen::core::log::debug!(class = stringify!(#type_name),"registering class");
                submit_register_callback(#type_name::js_init);
            }
        }
    }
}

/// find methods which are defined in node_bindgen annotation
fn generate_properties(class: &Class) -> Vec<TokenStream> {
    class
        .methods
        .iter()
        .filter_map(|method| {
            if method.attributes.is_constructor() {
                None
            } else {
                let method_ident = &method.method_name();

                let property_name = method.property_name();
                let napi_name = ident(&format!("napi_{method_ident}"));

                Some(if method.attributes.is_getter() {
                    quote! {
                        node_bindgen::core::Property::new(#property_name).getter(Self::#napi_name)
                    }
                } else if method.attributes.is_setter() {
                    quote! {
                        node_bindgen::core::Property::new(#property_name).setter(Self::#napi_name)
                    }
                } else {
                    quote! {
                        node_bindgen::core::Property::new(#property_name).method(Self::#napi_name)
                    }
                })
            }
        })
        .collect()
}
