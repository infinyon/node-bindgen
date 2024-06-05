use syn::Ident;
use syn::TypePath;
use syn::ImplItemMethod;
use syn::Attribute;
use syn::PathArguments;
use syn::Lifetime;
use syn::GenericArgument;

pub trait TypePathUtil {
    fn name_identifier(&self) -> Option<&Ident>;
    fn lifetime(&self) -> Option<&Lifetime>;
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
    /// find lifetime name.
    fn lifetime(&self) -> Option<&Lifetime> {
        let first = self.path.segments.first()?;
        let lifetime_arg = if let PathArguments::AngleBracketed(arguments) = &first.arguments {
            arguments.args.first()?
        } else {
            return None;
        };
        let lifetime = if let GenericArgument::Lifetime(lifetime) = lifetime_arg {
            lifetime
        } else {
            return None;
        };
        Some(lifetime)
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
