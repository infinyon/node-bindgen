use quote::quote;
use syn::Ident;
use syn::TypePath;
use syn::Type;
use syn::TypeReference;
use proc_macro2::Span;
use proc_macro2::TokenStream;

pub struct MyTypePath(TypePath);

impl MyTypePath {

    /// given this, convert into normalized type signature
    pub fn from(ty: Box<Type>) -> Option<Self> {

        match *ty {
            Type::Path(path_type) => Some(MyTypePath(path_type)),
            _ => None
        }
    }

    pub fn new(path_type: TypePath) -> Self {

        Self(path_type)
    }

    pub fn inner(&self) -> &TypePath {
        &self.0
    }

    /// return possible type name
    pub fn type_name(&self) -> Option<Ident> {

        for segment in &self.0.path.segments {
            return Some(segment.ident.clone());   
        }
        None
    }

    /// generate code as part of invoking rust function
    /// for normal argument, it is just variable
    /// other may like closure may need to convert to rust closure
    pub fn as_arg_token_stream(&self,index: usize) -> TokenStream {

        let var_name = rust_arg_var(index);

        quote! {
            #var_name,
        }
    }

}



/// rust argument
pub fn rust_arg_var(index: usize) -> Ident {
    let var_name = format!("rust_value_{}", index);
    Ident::new(&var_name, Span::call_site())
}



pub struct MyReferenceType(TypeReference);

impl MyReferenceType {
    pub fn new(ty: TypeReference) -> Self {
        Self(ty)
    }


    pub fn is_callback(&self) -> bool {
        match &*self.0.elem {
            Type::Path(path_type) => {
                path_type.path.segments.iter().find(|segment| segment.ident == "JsCallback").is_some()
            },
            _ => false
        }
    }

    /// generate code as part of invoking rust function
    pub fn as_arg_token_stream(&self,index: usize) -> TokenStream {

        let var_name = rust_arg_var(index);

        quote! {
            #var_name,
        }
    }

    /// return possible type name
    pub fn type_name(&self) -> Option<Ident> {

        match self.0.elem.as_ref() {
            Type::Path(path) => {

                for segment in &path.path.segments {
                    return Some(segment.ident.clone());   
                }
                None

            },
            _ => None
        }

        
    }
}

/// generate default property name for function which uses camel case
pub fn default_function_property_name(fn_name: &str) -> String {
    use inflector::Inflector;

    fn_name.to_camel_case()
}