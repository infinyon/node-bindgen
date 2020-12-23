use syn::ItemFn;
use syn::Ident;
use syn::TypePath;
use syn::ImplItemMethod;
use syn::Attribute;

/// traits for function item
pub trait FunctionItem {
    fn name(&self) -> &Ident;
}

impl FunctionItem for ItemFn {
    fn name(&self) -> &Ident {
        &self.sig.ident
    }
}

pub trait TypePathUtil {
    fn name_identifier(&self) -> Option<&Ident>;
}

impl TypePathUtil for TypePath {
    /// find name identifier
    fn name_identifier(&self) -> Option<&Ident> {
        self.path
            .segments
            .iter()
            .find(|_| true)
            .map(|segment| &segment.ident)
    }
}

pub trait MethodUtil {
    fn find_attr(&self) -> Option<&Attribute>;
}

impl MethodUtil for ImplItemMethod {
    /// find attr that contains node bindgen
    fn find_attr(&self) -> Option<&Attribute> {
        self.attrs.iter().find(|attr| {
            attr.path
                .segments
                .iter()
                .any(|seg| seg.ident == "node_bindgen")
        })
    }
}
