use proc_macro2::Span;
use syn::ItemImpl;
use syn::ImplItem;
use syn::Result;
use syn::Ident;
use syn::LitStr;
use syn::Error;
use syn::ImplItemMethod;
use syn::spanned::Spanned;

use crate::ast::MethodUtil;
use crate::util::default_function_property_name;
use super::FunctionAttributes;
use super::FunctionArgs;
use super::MyTypePath;

pub struct Class<'a> {
    pub self_ty: MyTypePath<'a>,
    pub methods: Vec<Method<'a>>,
}

impl<'a> Class<'a> {
    /// convert
    pub fn from_ast(item: &'a ItemImpl) -> Result<Self> {
        use syn::Type;

        let mut methods = vec![];
        for item in &item.items {
            if let ImplItem::Method(method) = item {
                if let Some(method) = Method::from_ast(method)? {
                    methods.push(method);
                }
            }
        }

        // find type path
        let self_ty = match &*item.self_ty {
            Type::Path(path_type) => MyTypePath::from(path_type)?,
            _ => return Err(Error::new(item.span(), "not supported receiver type")),
        };

        Ok(Self { self_ty, methods })
    }

    pub fn constructor(&'a self) -> Option<&'a Method> {
        self.methods
            .iter()
            .find(|method| method.attributes.is_constructor())
    }

    pub fn my_type(&'a self) -> &MyTypePath<'a> {
        &self.self_ty
    }
}

pub struct Method<'a> {
    pub method: &'a ImplItemMethod,
    pub attributes: FunctionAttributes,
    pub args: FunctionArgs<'a>,
}

impl<'a> Method<'a> {
    /// extract js method, if it can't find marker attribute, return some
    pub fn from_ast(method: &'a ImplItemMethod) -> Result<Option<Self>> {
        if let Some(attr) = method.find_attr() {
            let args = FunctionArgs::from_ast(&method.sig)?;
            Ok(Some(Self {
                method,
                args,
                attributes: FunctionAttributes::from_method_attribute(attr)?,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn method_name(&self) -> &Ident {
        &self.method.sig.ident
    }

    /// used for registering in the Napi
    pub fn property_name(&self) -> LitStr {
        if let Some(name) = self.attributes.name() {
            LitStr::new(name, Span::call_site())
        } else {
            LitStr::new(
                &default_function_property_name(&self.method_name().to_string()),
                Span::call_site(),
            )
        }
    }
}
