use quote::{quote, format_ident};
use proc_macro2::TokenStream;
use proc_macro2::Span;

use syn::DeriveInput;
use syn::Result;
use syn::Data;
use syn::DataStruct;
use syn::Fields;
use syn::FieldsNamed;
use syn::FieldsUnnamed;
use syn::Type;
use syn::Ident;
use syn::Index;
use syn::Error;
use syn::spanned::Spanned;
use syn::GenericParam;
use syn::TypeParam;
use syn::LifetimeDef;
use syn::ConstParam;
use syn::WhereClause;
use syn::punctuated::Punctuated;

use crate::ast::MyTypePath;
use crate::ast::MyReferenceType;
use crate::ast::MyStruct;
use crate::ast::MyField;
use crate::ast::MyFieldType;
use crate::ast::MyGenerics;

pub fn generate_datatype(input_struct: DeriveInput) -> TokenStream {
    match MyStruct::from_ast(&input_struct) {
        Err(err) => err.to_compile_error(),
        Ok(parsed_struct) => {
            println!("We got struct: {:?}", parsed_struct);

            let try_into_js = generate_try_into_js(&parsed_struct);
            let result = quote! {
                // We are in a datatype gen
                #input_struct

                #try_into_js
            };
            println!("Got {}", result.to_string());
            result
        }
    }
}

fn drop_generic_bounds(params: &Vec<GenericParam>) -> Vec<GenericParam> {
    params
        .clone()
        .into_iter()
        .map(|generic| {
            match generic {
                GenericParam::Type(type_param) => {
                    GenericParam::Type(TypeParam {
                        colon_token: None,
                        bounds: Punctuated::new(),
                        ..type_param
                    })
                },
                GenericParam::Lifetime(lifetime_param) => {
                    GenericParam::Lifetime(LifetimeDef {
                        colon_token: None,
                        bounds: Punctuated::new(),
                        ..lifetime_param
                    })
                },
                GenericParam::Const(const_param) => {
                    GenericParam::Const(ConstParam {
                        eq_token: None,
                        default: None,
                        ..const_param
                    })
                }
            }
        })
        .collect()
}

fn generate_try_into_js(parsed_struct: &MyStruct) -> TokenStream {
    match parsed_struct {
        MyStruct::Named { name, fields, generics } => {
            let impl_signature = generate_impl_signature(name, generics);
            let output_obj     = format_ident!("output_obj");
            let js_env         = format_ident!("js_env");

            let field_conversions = generate_named_field_conversions(
                &output_obj,
                &js_env,
                fields);

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
        MyStruct::Unnamed { name, fields, generics } => {
            let impl_signature = generate_impl_signature(name, generics);
            let fields_count   = fields.len();
            let output_arr     = format_ident!("output_arr");
            let js_env         = format_ident!("js_env");
            let field_conversions = generate_unnamed_field_conversions(
                &output_arr,
                &js_env,
                fields);

            quote! {
                #impl_signature {
                    fn try_to_js(self, #js_env: &node_bindgen::core::val::JsEnv) ->
                        Result<node_bindgen::core::sys::napi_value, 
                            node_bindgen::core::NjError> 
                    {
                        use node_bindgen::core::{
                            TryIntoJs
                        };

                        let #output_arr = js_env.create_array_with_len(#fields_count)?;

                        #(#field_conversions)*
                        
                        Ok(#output_arr)
                    }
                }
            }
        },
        MyStruct::Unit { name } => {
            quote! {
                impl node_bindgen::core::TryIntoJs for #name {
                    fn try_to_js(self, js_env: &node_bindgen::core::val::JsEnv) ->
                        Result<node_bindgen::core::sys::napi_value,
                               node_bindgen::core::NjError>
                    {
                        node_bindgen::core::val::JsObject::new(
                            js_env.clone(),
                            js_env.create_object()?
                        )
                        .try_to_js(js_env)
                    }
                }
            }
        }
    }
}

fn generate_impl_signature<'a>(name: &'a Ident, generics: &'a MyGenerics<'a>) -> TokenStream
{
    let generic_params     = &generics.params;
    let generics_no_bounds = drop_generic_bounds(&generics.params);
    let where_clause       = match generics.where_clause {
        None => quote! {},
        Some(where_clause) => quote! {
            #where_clause
        }
    };

    quote! {
        impl <#(#generic_params),*> node_bindgen::core::TryIntoJs for 
                #name<#(#generics_no_bounds),*> #where_clause
    }
}

fn generate_named_field_conversions<'a>(
                                  output_obj: &Ident, 
                                  js_env: &Ident,
                                  fields: &'a Vec<MyField<'a>>) -> Vec<TokenStream> {
    fields.iter()
        .map(|MyField { name, ty: _ }| {
            let field_name = format!("{}", name);

            quote! {
                #output_obj.set_property(
                    #field_name, 
                    self.#name.clone().try_to_js(#js_env)?)?;
            }
        })
        .collect()
}

fn generate_unnamed_field_conversions<'a>(
                                  output_array: &Ident, 
                                  js_env: &Ident,
                                  fields: &'a Vec<MyFieldType<'a>>) -> Vec<TokenStream> {
    fields.iter()
        .enumerate()
        .map(|(field_idx, _)| {
            let index = Index {
                index: field_idx as u32,
                span: output_array.span()
            };

            quote! {
                #js_env.set_element(
                    #output_array, 
                    self.#index.clone().try_to_js(#js_env)?,
                    #index)?;
            }
        })
        .collect()
}
