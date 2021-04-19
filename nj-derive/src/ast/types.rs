use syn::Ident;
use syn::TypePath;
use syn::Type;
use syn::TypeReference;
use syn::Error;
use syn::Result;
use syn::spanned::Spanned;
use quote::quote;
use proc_macro2::TokenStream;

use syn::DeriveInput;
use syn::Data;
use syn::DataStruct;
use syn::Fields;
use syn::FieldsNamed;
use syn::FieldsUnnamed;
use syn::Index;
use syn::GenericParam;
use syn::TypeParam;
use syn::LifetimeDef;
use syn::ConstParam;
use syn::WhereClause;
use syn::punctuated::Punctuated;

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
pub enum MyStruct<'a> {
    Named {
        name: &'a Ident,
        fields: Vec<MyField<'a>>,
        generics: MyGenerics<'a>
    },
    Unnamed {
        name: &'a Ident,
        fields: Vec<MyFieldType<'a>>,
        generics: MyGenerics<'a>
    },
    Unit {
        name: &'a Ident
    }
}

#[derive(Debug)]
pub struct MyField<'a> {
    pub name: &'a Ident,
    pub ty: MyFieldType<'a>
}

#[derive(Debug)]
pub enum MyFieldType<'a> {
    Path(MyTypePath<'a>),
    Ref(MyReferenceType<'a>),
}

#[derive(Debug)]
pub struct MyGenerics<'a> {
    pub params: Vec<GenericParam>,
    pub where_clause: &'a Option<WhereClause>
}

impl<'a> MyFieldType<'a> {
    pub fn from(ty: &'a Type) -> Result<Self> {
        match ty {
            Type::Path(type_path) => Ok(MyFieldType::Path(
                MyTypePath::from(type_path)?)),
            Type::Reference(reference) => Ok(MyFieldType::Ref(
                MyReferenceType::from(reference)?)),
            _ => Err(Error::new(ty.span(), "Only type paths and references \
                    are supported as field types")),
        }
    }
}

impl<'a> MyStruct<'a> {
    pub fn from_ast(input: &'a DeriveInput) -> Result<MyStruct> {
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
        let generics = MyGenerics {
            params: generic_params,
            where_clause: &input.generics.where_clause
        };

        match &struct_data.fields {
            Fields::Named(named_fields) => {
                let fields = named_fields.named
                    .iter()
                    .filter_map(|field| field.ident.as_ref().map(|ident| (ident, &field.ty)))
                    .map(|(ident, ty)| {
                        MyFieldType::from(&ty)
                            .map(|ty| MyField {
                                name: &ident,
                                ty
                            })
                    })
                    .collect::<Result<Vec<MyField<'a>>>>()?;

                Ok(MyStruct::Named {
                    name: &input.ident,
                    fields,
                    generics,
                })
            },
            Fields::Unnamed(unnamed_fields) => {
                let fields = unnamed_fields.unnamed
                    .iter()
                    .map(|field| MyFieldType::from(&field.ty))
                    .collect::<Result<Vec<MyFieldType<'a>>>>()?;

                Ok(MyStruct::Unnamed {
                    name: &input.ident,
                    fields,
                    generics,
                })
            },
            Fields::Unit => Ok(MyStruct::Unit { name: &input.ident })
        }
    }
}