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

impl <'a>MyTypePath<'a> {


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
pub struct MyReferenceType<'a>(&'a Ident);

impl <'a> MyReferenceType<'a> {


    pub fn from(ty: &'a TypeReference) -> Result<Self> {

        match ty.elem.as_ref() {
            Type::Path(path) => {

                if let Some(name_id) = path.name_identifier() {
                    Ok(Self(name_id))
                } else {
                    Err(Error::new(ty.span(), "no type identifier found"))
                }
            },
            _ => Err(Error::new(ty.span(), "no type identifier found"))
        }
    }



    /// return any first one
    pub fn type_name(&self) -> &Ident {
        self.0
    }


}