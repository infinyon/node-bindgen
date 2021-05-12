use quote::{quote, format_ident};
use proc_macro2::TokenStream;
use syn::DeriveInput;
use syn::Ident;
use syn::Index;
use syn::GenericParam;
use syn::TypeParam;
use syn::LifetimeDef;
use syn::ConstParam;
use syn::punctuated::Punctuated;

use inflector::Inflector;

use crate::ast::MyStruct;
use crate::ast::MyEnum;
use crate::ast::MyFields;
use crate::ast::MyFieldType;
use crate::ast::MyNamedField;
use crate::ast::MyUnnamedField;
use crate::ast::MyGenerics;
use crate::ast::MyDeriveInput;
use crate::ast::MyDerivePayload;
use crate::ast::MyVariant;

pub fn generate_datatype(input_data: DeriveInput) -> TokenStream {
    match MyDeriveInput::from_ast(&input_data) {
        Err(err) => err.to_compile_error(),
        Ok(parsed_data) => {
            let try_into_js = generate_try_into_js(&parsed_data);
            quote! {
                #input_data

                #try_into_js
            }
        }
    }
}


fn generate_try_into_js(parsed_data: &MyDeriveInput) -> TokenStream {
    let impl_signature = generate_impl_signature(&parsed_data.name, &parsed_data.generics);

    match &parsed_data.payload {
        MyDerivePayload::Struct(struct_data) => {
            generate_struct_try_into_js(&impl_signature, &struct_data)
        },
        MyDerivePayload::Enum(enum_data) => {
            generate_enum_try_into_js(&parsed_data.name, &impl_signature, &enum_data)
        }
    }
}

fn generate_enum_try_into_js(enum_name: &Ident, impl_signature: &TokenStream, enum_data: &MyEnum) -> TokenStream {
    let js_env = format_ident!("js_env");

    let variant_conversions = enum_data.variants.iter()
        .map(|v| generate_variant_conversion(enum_name, &js_env, v))
        .collect::<Vec<TokenStream>>();

    quote! {
        #impl_signature {
            fn try_to_js(self, #js_env: &node_bindgen::core::val::JsEnv) ->
                Result<node_bindgen::core::sys::napi_value,
                       node_bindgen::core::NjError>
            {
                use node_bindgen::core::{
                    TryIntoJs,
                    val::JsObject
                };

                match self {
                    #(#variant_conversions),*
                }
            }
        }
    }
}

fn generate_variant_conversion(enum_name: &Ident, js_env: &Ident, variant: &MyVariant) -> TokenStream {
    let variant_name           = variant.name;
    let variant_name_camelcase = format!("{}", variant.name).to_camel_case();
    let fields_scope           = quote! {};
    let output_obj = format_ident!("output_obj");

    match &variant.fields {
        MyFields::Named(named_fields) => {
            let variant_output_obj = format_ident!("variant_output_obj");
            let field_bindings = named_fields.iter()
                .map(|field| field.name)
                .collect::<Vec<&Ident>>();

            let field_conversions = generate_named_field_conversions(&variant_output_obj,
                                                                     &fields_scope,
                                                                     &js_env,
                                                                     &named_fields);

            quote! {
                #enum_name::#variant_name { #(#field_bindings),* } => {
                    let mut #output_obj = JsObject::new(#js_env.clone(),
                        #js_env.create_object()?);

                    let mut #variant_output_obj = JsObject::new(#js_env.clone(),
                        #js_env.create_object()?);

                    #(#field_conversions)*

                    #output_obj.set_property(
                        #variant_name_camelcase,
                        #variant_output_obj.try_to_js(#js_env)?
                    )?;

                    #output_obj.try_to_js(#js_env)
                }
            }
        },
        MyFields::Unnamed(unnamed_fields) => {
            let variant_output_arr = format_ident!("variant_output_arr");
            let fields_count       = unnamed_fields.len();
            let field_bindings = (0..fields_count)
                .into_iter()
                .map(|field_idx| format_ident!("field_{}", field_idx))
                .collect::<Vec<Ident>>();

            let field_conversions = generate_bound_unnamed_field_conversions(&variant_output_arr,
                                                                             &js_env,
                                                                             &field_bindings);

            quote! {
                #enum_name::#variant_name( #(#field_bindings),* ) => {
                    let mut #output_obj = JsObject::new(#js_env.clone(),
                        #js_env.create_object()?);

                    let #variant_output_arr = #js_env.create_array_with_len(#fields_count)?;

                    #(#field_conversions)*

                    #output_obj.set_property(
                        #variant_name_camelcase,
                        #variant_output_arr.try_to_js(#js_env)?
                    )?;

                    #output_obj.try_to_js(#js_env)
                }
            }
        },
        MyFields::Unit => {
            quote! {
                #enum_name::#variant_name => {
                    #js_env.create_string_utf8(#variant_name_camelcase)
                }
            }
        }
    }
}

fn generate_struct_try_into_js(impl_signature: &TokenStream, struct_data: &MyStruct) -> TokenStream {
    let js_env = format_ident!("js_env");
    let fields_scope = quote! {
        self.
    };


    match &struct_data.fields {
        MyFields::Named(named_fields) => {
            let output_obj = format_ident!("output_obj");
            let field_conversions =
                generate_named_field_conversions(&output_obj,
                                                 &fields_scope,
                                                 &js_env,
                                                 &named_fields);

            quote! {
                #impl_signature {
                    fn try_to_js(self, #js_env: &node_bindgen::core::val::JsEnv) ->
                        Result<node_bindgen::core::sys::napi_value,
                            node_bindgen::core::NjError>
                    {
                        use node_bindgen::core::{
                            TryIntoJs,
                            val::JsObject
                        };

                        let mut #output_obj = JsObject::new(#js_env.clone(),
                            #js_env.create_object()?);

                        #(#field_conversions)*

                        #output_obj.try_to_js(#js_env)
                    }
                }
            }
        },
        MyFields::Unnamed(unnamed_fields) => {
            let fields_count = unnamed_fields.len();
            let output_arr = format_ident!("output_arr");
            let field_conversions =
                generate_unnamed_field_conversions(&output_arr,
                                                   &js_env,
                                                   &unnamed_fields);

            quote! {
                #impl_signature {
                    fn try_to_js(self, #js_env: &node_bindgen::core::val::JsEnv) ->
                        Result<node_bindgen::core::sys::napi_value,
                            node_bindgen::core::NjError>
                    {
                        use node_bindgen::core::{
                            TryIntoJs
                        };

                        let #output_arr = #js_env.create_array_with_len(#fields_count)?;

                        #(#field_conversions)*

                        Ok(#output_arr)
                    }
                }
            }
        }
        MyFields::Unit => {
            quote! {
                #impl_signature {
                    fn try_to_js(self, #js_env: &node_bindgen::core::val::JsEnv) ->
                        Result<node_bindgen::core::sys::napi_value,
                            node_bindgen::core::NjError>
                    {
                        #js_env.get_null()
                    }
                }
            }
        }
    }
}

fn drop_generic_bounds(params: &[GenericParam]) -> Vec<GenericParam> {
    params
        .to_owned()
        .into_iter()
        .map(|generic| match generic {
            GenericParam::Type(type_param) => GenericParam::Type(TypeParam {
                colon_token: None,
                bounds: Punctuated::new(),
                ..type_param
            }),
            GenericParam::Lifetime(lifetime_param) => GenericParam::Lifetime(LifetimeDef {
                colon_token: None,
                bounds: Punctuated::new(),
                ..lifetime_param
            }),
            GenericParam::Const(const_param) => GenericParam::Const(ConstParam {
                eq_token: None,
                default: None,
                ..const_param
            }),
        })
        .collect()
}

fn generate_impl_signature<'a>(name: &'a Ident, generics: &'a MyGenerics<'a>) -> TokenStream {
    let generic_params = &generics.params;
    let generics_no_bounds = drop_generic_bounds(&generics.params);
    let where_clause = match generics.where_clause {
        None => quote! {},
        Some(where_clause) => quote! {
            #where_clause
        },
    };

    quote! {
        impl <#(#generic_params),*> node_bindgen::core::TryIntoJs for
                #name<#(#generics_no_bounds),*> #where_clause
    }
}

fn generate_named_field_conversions<'a>(
    output_obj: &Ident,
    fields_scope: &TokenStream,
    js_env: &Ident,
    fields: &'a [MyNamedField<'a>],
) -> Vec<TokenStream> {
    fields
        .iter()
        .map(|MyNamedField { name, ty }| {
            let field_name = format!("{}", name).to_camel_case();

            // References needs to be cloned for try_to_js
            // to take their ownership. Values can be passed as is
            let field_access = match ty {
                MyFieldType::Path(_) => quote! { #fields_scope #name },
                MyFieldType::Ref(_) => quote! { #fields_scope #name.clone() },
            };

            quote! {
                #output_obj.set_property(
                    #field_name,
                    #field_access.try_to_js(#js_env)?)?;
            }
        })
        .collect()
}

fn generate_unnamed_field_conversions<'a>(
    output_array: &Ident,
    js_env: &Ident,
    fields: &'a [MyUnnamedField<'a>],
) -> Vec<TokenStream> {
    fields
        .iter()
        .enumerate()
        .map(|(field_idx, MyUnnamedField { ty })| {
            let index = Index {
                index: field_idx as u32,
                span: output_array.span(),
            };

            let field_access = match ty {
                MyFieldType::Path(_) => quote! { self.#index },
                MyFieldType::Ref(_) => quote! { self.#index.clone() },
            };

            quote! {
                #js_env.set_element(
                    #output_array,
                    #field_access.try_to_js(#js_env)?,
                    #index)?;
            }
        })
        .collect()
}

fn generate_bound_unnamed_field_conversions<'a>(
    output_array: &Ident,
    js_env: &Ident,
    field_bindings: &'a [Ident]
) -> Vec<TokenStream> {
    field_bindings
        .iter()
        .enumerate()
        .map(|(field_idx, field_ident)| {
            let index = Index {
                index: field_idx as u32,
                span: output_array.span(),
            };

            quote! {
                #js_env.set_element(
                    #output_array,
                    #field_ident.try_to_js(#js_env)?,
                    #index)?;
            }
        })
        .collect()
}