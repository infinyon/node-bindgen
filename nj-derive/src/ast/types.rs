use syn::Ident;
use syn::TypePath;
use syn::Type;
use syn::TypeReference;
use syn::Error;
use syn::Result;
use syn::spanned::Spanned;
use syn::DeriveInput;
use syn::Data;
use syn::DataEnum;
use syn::DataStruct;
use syn::Variant;
use syn::Fields;
use syn::GenericParam;
use syn::WhereClause;
use quote::quote;
use proc_macro2::TokenStream;

use crate::ast::TypePathUtil;

#[derive(Debug)]
pub struct MyTypePath<'a>(&'a TypePath);

impl<'a> MyTypePath<'a> {
    pub fn from(ty: &'a TypePath) -> Result<Self> {
        Ok(Self(ty))
    }

    pub fn ident(&self) -> Option<&Ident> {
        self.0.name_identifier()
    }
    pub fn lifetime(&self) -> Option<TokenStream> {
        let ty = self.0.lifetime()?;
        Some(quote! {
            #ty
        })
    }

    pub fn expansion(&self) -> TokenStream {
        let ty = self.0;
        quote! {
            #ty
        }
    }
}

#[derive(Debug)]
pub struct MyReferenceType<'a> {
    ident: &'a Ident,
    inner: &'a TypeReference,
}

impl<'a> MyReferenceType<'a> {
    pub fn from(ty: &'a TypeReference) -> Result<Self> {
        //println!("tye: {:#?}", ty);
        Ok(Self {
            ident: get_type_name(ty.elem.as_ref())?,
            inner: ty,
        })
    }

    /// return any first one
    #[allow(unused)]
    pub fn type_name(&self) -> &Ident {
        self.ident
    }

    pub fn expansion(&self) -> TokenStream {
        let ty = self.inner;
        quote! {
            #ty
        }
    }
}

fn get_type_name(ty: &Type) -> Result<&Ident> {
    match ty {
        Type::Path(path) => {
            if let Some(name_id) = path.name_identifier() {
                Ok(name_id)
            } else {
                Err(Error::new(
                    ty.span(),
                    "no named type identifier found for type path",
                ))
            }
        }
        Type::Slice(slice) => get_type_name(&slice.elem),
        _ => Err(Error::new(ty.span(), "no other type reference found")),
    }
}

#[derive(Debug)]
pub struct MyTupleType<'a> {
    types: Vec<&'a Type>,
}

impl<'a> From<Vec<&'a Type>> for MyTupleType<'a> {
    fn from(types: Vec<&'a Type>) -> Self {
        Self { types }
    }
}

impl MyTupleType<'_> {
    pub fn expansion(&self) -> TokenStream {
        let types = &self.types;
        quote! {
            ( #( #types ),* )
        }
    }
}

#[derive(Debug)]
pub struct MyDeriveInput<'a> {
    pub name: &'a Ident,
    pub generics: MyGenerics<'a>,
    pub payload: MyDerivePayload<'a>,
}

#[derive(Debug)]
pub enum MyDerivePayload<'a> {
    Struct(MyStruct<'a>),
    Enum(MyEnum<'a>),
}

impl<'a> MyDeriveInput<'a> {
    pub fn from_ast(input: &'a DeriveInput) -> Result<MyDeriveInput> {
        let name = &input.ident;
        let generic_params = input.generics.params.clone().into_iter().collect();
        let generics = MyGenerics {
            params: generic_params,
            where_clause: &input.generics.where_clause,
        };

        match &input.data {
            Data::Struct(inner_struct) => {
                let parsed_struct = MyStruct::from_ast(&inner_struct)?;
                Ok(MyDeriveInput {
                    name,
                    generics,
                    payload: MyDerivePayload::Struct(parsed_struct),
                })
            }
            Data::Enum(inner_enum) => {
                let parsed_enum = MyEnum::from_ast(&inner_enum)?;
                Ok(MyDeriveInput {
                    name,
                    generics,
                    payload: MyDerivePayload::Enum(parsed_enum),
                })
            }
            Data::Union(_) => Err(Error::new(
                input.span(),
                "Unions are not supported \
                for automatic conversion to JavaScript representation",
            )),
        }
    }
}

#[derive(Debug)]
pub enum MyFields<'a> {
    Named(Vec<MyNamedField<'a>>),
    Unnamed(Vec<MyUnnamedField<'a>>),
    Unit,
}

#[derive(Debug)]
pub struct MyNamedField<'a> {
    pub name: &'a Ident,
    pub ty: MyFieldType<'a>,
}

#[derive(Debug)]
pub struct MyUnnamedField<'a> {
    pub ty: MyFieldType<'a>,
}

#[derive(Debug)]
pub enum MyFieldType<'a> {
    Path(MyTypePath<'a>),
    Ref(MyReferenceType<'a>),
}

impl<'a> MyFields<'a> {
    pub fn from_ast(input: &'a Fields) -> Result<MyFields> {
        match &input {
            Fields::Named(named_fields) => {
                let fields = named_fields
                    .named
                    .iter()
                    .filter_map(|field| field.ident.as_ref().map(|ident| (ident, &field.ty)))
                    .map(|(ident, ty)| {
                        MyFieldType::from(&ty).map(|ty| MyNamedField { name: &ident, ty })
                    })
                    .collect::<Result<Vec<MyNamedField<'a>>>>()?;

                Ok(MyFields::Named(fields))
            }
            Fields::Unnamed(unnamed_fields) => {
                let fields = unnamed_fields
                    .unnamed
                    .iter()
                    .map(|field| MyFieldType::from(&field.ty))
                    .collect::<Result<Vec<MyFieldType<'a>>>>()?
                    .into_iter()
                    .map(|ty| MyUnnamedField { ty })
                    .collect::<Vec<MyUnnamedField<'a>>>();

                Ok(MyFields::Unnamed(fields))
            }
            Fields::Unit => Ok(MyFields::Unit),
        }
    }
}

impl<'a> MyFieldType<'a> {
    pub fn from(ty: &'a Type) -> Result<MyFieldType> {
        match ty {
            Type::Path(type_path) => Ok(MyFieldType::Path(MyTypePath::from(type_path)?)),
            Type::Reference(reference) => Ok(MyFieldType::Ref(MyReferenceType::from(reference)?)),
            _ => Err(Error::new(
                ty.span(),
                "Only type paths and references \
                    are supported as field types",
            )),
        }
    }
}

#[derive(Debug)]
pub struct MyEnum<'a> {
    pub variants: Vec<MyVariant<'a>>,
}

impl<'a> MyEnum<'a> {
    pub fn from_ast(enum_data: &'a DataEnum) -> Result<MyEnum> {
        let variants = enum_data
            .variants
            .iter()
            .map(|v| MyVariant::from_ast(v))
            .collect::<Result<Vec<MyVariant>>>()?;

        Ok(MyEnum { variants })
    }
}

#[derive(Debug)]
pub struct MyVariant<'a> {
    pub name: &'a Ident,
    pub fields: MyFields<'a>,
}

impl<'a> MyVariant<'a> {
    pub fn from_ast(variant_data: &'a Variant) -> Result<MyVariant> {
        let fields = MyFields::from_ast(&variant_data.fields)?;

        Ok(MyVariant {
            name: &variant_data.ident,
            fields,
        })
    }
}

#[derive(Debug)]
pub struct MyStruct<'a> {
    pub fields: MyFields<'a>,
}

impl<'a> MyStruct<'a> {
    pub fn from_ast(struct_data: &'a DataStruct) -> Result<MyStruct> {
        let fields = MyFields::from_ast(&struct_data.fields)?;

        Ok(MyStruct { fields })
    }
}

#[derive(Debug)]
pub struct MyGenerics<'a> {
    pub params: Vec<GenericParam>,
    pub where_clause: &'a Option<WhereClause>,
}
