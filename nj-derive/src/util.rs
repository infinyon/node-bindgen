use proc_macro2::Span;
use syn::LitStr;

/// generate default property name for function which uses camel case
pub fn default_function_property_name(fn_name: &str) -> String {
    use inflector::Inflector;

    fn_name.to_camel_case()
}

pub fn arg_ident(index: usize) -> syn::Ident {
    ident(&format!("arg{index}"))
}

pub fn ident(ident: &str) -> syn::Ident {
    syn::Ident::new(ident, Span::call_site())
}

pub fn lit_str(ident: &str) -> LitStr {
    LitStr::new(ident, Span::call_site())
}
