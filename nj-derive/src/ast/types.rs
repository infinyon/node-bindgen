use syn::Ident;
use syn::TypePath;
use syn::Type;
use syn::TypeReference;
use syn::Error;
use syn::Result;
use syn::spanned::Spanned;
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

    pub fn expansion(&self) -> TokenStream {
        let ty = self.0;
        quote! {
            #ty
        }
    }
}

#[derive(Debug)]
pub struct MyReferenceType<'a>{
    ident: &'a Ident,
    inner: &'a TypeReference
}

impl<'a> MyReferenceType<'a> {
    pub fn from(ty: &'a TypeReference) -> Result<Self> {
        //println!("tye: {:#?}", ty);
        Ok(Self{
            ident: get_type_name(ty.elem.as_ref())?,
            inner: ty
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
