use quote::quote;
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

pub fn generate_datatype(input_struct: DeriveInput) -> TokenStream {
    match Struct::from_ast(&input_struct) {
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

fn generate_try_into_js(parsed_struct: &Struct) -> TokenStream {
    match parsed_struct {
        Struct::Named { name, fields, generics } => {
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
                        #name<#(#generics_no_bounds),*> #where_clause {
                    fn try_to_js(self, js: &node_bindgen::core::val::JsEnv) ->
                        Result<node_bindgen::core::sys::napi_value, 
                               node_bindgen::core::NjError> 
                    {
                        unimplemented!()
                    }
                }
            }
        },
        _ => unimplemented!()
    }
}

#[derive(Debug)]
pub enum Struct<'a> {
    Named {
        name: &'a Ident,
        fields: Vec<Field<'a>>,
        generics: Generics<'a>
    },
    Unnamed {
        name: &'a Ident,
        fields: Vec<FieldType<'a>>,
        generics: Generics<'a>
    },
    Unit {
        name: &'a Ident
    }
}

#[derive(Debug)]
pub struct Field<'a> {
    name: &'a Ident,
    ty: FieldType<'a>
}

#[derive(Debug)]
pub enum FieldType<'a> {
    Path(MyTypePath<'a>),
    Ref(MyReferenceType<'a>),
}

#[derive(Debug)]
pub struct Generics<'a> {
    params: Vec<GenericParam>,
    where_clause: &'a Option<WhereClause>
}

impl<'a> FieldType<'a> {
    pub fn from(ty: &'a Type) -> Result<Self> {
        match ty {
            Type::Path(type_path) => Ok(FieldType::Path(
                MyTypePath::from(type_path)?)),
            Type::Reference(reference) => Ok(FieldType::Ref(
                MyReferenceType::from(reference)?)),
            _ => Err(Error::new(ty.span(), "Only type paths and references \
                    are supported as field types")),
        }
    }
}

impl<'a> Struct<'a> {
    pub fn from_ast(input: &'a DeriveInput) -> Result<Struct> {
        let struct_data = match &input.data {
            Data::Struct(inner_struct) => Ok(inner_struct),
            Data::Enum(_) => Err(Error::new(input.span(), "Enums are not supported \
                for automatic conversion to JavaScript representation")),
            Data::Union(_) => Err(Error::new(input.span(), "Unions are not supported \
                for automatic conversion to JavaScript representation")),
        }?;

        let generic_params = input.generics.params
            .clone()
            .into_iter()
            .collect();
        let generics = Generics {
            params: generic_params,
            where_clause: &input.generics.where_clause
        };

        match &struct_data.fields {
            Fields::Named(named_fields) => {
                let fields = named_fields.named
                    .iter()
                    .filter_map(|field| field.ident.as_ref().map(|ident| (ident, &field.ty)))
                    .map(|(ident, ty)| {
                        FieldType::from(&ty)
                            .map(|ty| Field {
                                name: &ident,
                                ty
                            })
                    })
                    .collect::<Result<Vec<Field<'a>>>>()?;

                Ok(Struct::Named {
                    name: &input.ident,
                    fields,
                    generics,
                })
            },
            Fields::Unnamed(unnamed_fields) => {
                let fields = unnamed_fields.unnamed
                    .iter()
                    .map(|field| FieldType::from(&field.ty))
                    .collect::<Result<Vec<FieldType<'a>>>>()?;

                Ok(Struct::Unnamed {
                    name: &input.ident,
                    fields,
                    generics,
                })
            },
            Fields::Unit => Ok(Struct::Unit { name: &input.ident })
        }
    }
}