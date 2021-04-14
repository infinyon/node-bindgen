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

use crate::ast::MyTypePath;
use crate::ast::MyReferenceType;

pub fn generate_datatype(input_struct: DeriveInput) -> TokenStream {
    match Struct::from_ast(&input_struct) {
        Err(err) => err.to_compile_error(),
        Ok(parsed_struct) => {
            println!("We got struct: {:?}", parsed_struct);

            let try_into_js = generate_try_into_js(&parsed_struct);
            quote! {
                // We are in a datatype gen
                #input_struct

                #try_into_js
            }
        }
    }
}

fn generate_try_into_js(parsed_struct: &Struct) -> TokenStream {
    match parsed_struct {
        Struct::Named { name, fields, generics } => {
            quote! {
                impl TryIntoJs for #name {
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
        generics: Vec<&'a GenericParam>
    },
    Unnamed {
        name: &'a Ident,
        fields: Vec<FieldType<'a>>,
        generics: Vec<&'a GenericParam>
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

        let generics = input.generics.params
            .iter()
            .collect();

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