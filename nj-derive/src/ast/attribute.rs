use proc_macro2::Span;
use syn::AttributeArgs;
use syn::Attribute;
use syn::Result;
use syn::spanned::Spanned;
use syn::Error;
use syn::NestedMeta;
use syn::Meta;
use syn::MetaNameValue;
use syn::Lit;
use syn::LitStr;
use syn::Ident;
use syn::Path;

/// Represents bindgen attributes
/// Attribute can be attached to free standing function or impl block
/// name="my_function"
/// constructor
/// setter
/// mt
#[derive(Debug)]
pub enum FunctionAttribute {
    Getter(Ident),
    Setter(Ident),
    Constructor(Ident),
    Name(LitStr),
    Mt(Ident),
}

impl FunctionAttribute {
    fn from_ast(meta: Meta) -> Result<Self> {
        match meta {
            Meta::NameValue(name_value) => {
                if has_attribute(&name_value, "name") {
                    // check make sure name is str literal
                    match name_value.lit {
                        Lit::Str(str) => Ok(Self::Name(str)),
                        _ => Err(Error::new(
                            name_value.span(),
                            "name value is not string literal",
                        )),
                    }
                } else {
                    Err(Error::new(name_value.span(), "unsupported attribute:"))
                }
            }
            Meta::Path(p) => Self::from_ident(find_any_identifier(p)?),

            Meta::List(lit) => Err(Error::new(
                lit.span(),
                "nested attributes are not supported",
            )),
        }
    }

    fn from_ident(ident: Ident) -> Result<Self> {
        if ident == "constructor" {
            Ok(Self::Constructor(ident))
        } else if ident == "getter" {
            Ok(Self::Getter(ident))
        } else if ident == "setter" {
            Ok(Self::Setter(ident))
        } else if ident == "mt" {
            Ok(Self::Mt(ident))
        } else {
            Err(Error::new(ident.span(), "unrecognized attribute name"))
        }
    }

    fn is_constructor(&self) -> bool {
        matches!(self, Self::Constructor(_))
    }

    fn is_multi_threaded(&self) -> bool {
        matches!(self, Self::Mt(_))
    }

    /// get function name, if this is not name, return none
    fn fn_name(&self) -> Option<&LitStr> {
        match self {
            Self::Name(ref name) => Some(name),
            _ => None,
        }
    }

    fn is_getter(&self) -> bool {
        matches!(self, Self::Getter(_))
    }

    fn is_setter(&self) -> bool {
        matches!(self, Self::Setter(_))
    }
}

fn has_attribute(name_value: &MetaNameValue, attr_name: &str) -> bool {
    name_value
        .path
        .segments
        .iter()
        .any(|seg| seg.ident == attr_name)
}

fn find_any_identifier(path: Path) -> Result<Ident> {
    if path.segments.is_empty() {
        Err(Error::new(path.span(), "invalid attribute"))
    } else {
        Ok(path
            .segments
            .into_iter()
            .find(|_| true)
            .map(|segment| segment.ident)
            .unwrap())
    }
}

/// ast information related to attributes
#[derive(Debug, Default)]
pub struct FunctionAttributes {
    pub constructor: Option<FunctionAttribute>,
    pub multi_threaded: Option<FunctionAttribute>,
    pub getter: Option<FunctionAttribute>,
    pub setter: Option<FunctionAttribute>,
    name: Option<String>,
}

impl FunctionAttributes {
    pub fn from_ast(args: AttributeArgs) -> Result<Self> {
        //println!("attrs: {:#?}",args);
        let mut attrs: Vec<FunctionAttribute> = vec![];

        for attr in args {
            match attr {
                NestedMeta::Meta(meta) => {
                    attrs.push(FunctionAttribute::from_ast(meta)?);
                }
                _ => return Err(Error::new(attr.span(), "invalid syntax")),
            }
        }

        Ok(Self::from(attrs))
    }

    /// validate and parse attributes for individual features
    /// if check_method is true, check class specific attributes
    /// note that there are attribute parsing phases for class
    /// first phase is as part of Impl structure, this is where class level attribute validation can be done
    /// second phase is individual functions where we don't know if function is method or not
    fn from(attrs: Vec<FunctionAttribute>) -> Self {
        let mut constructor = None;
        let mut multi_threaded = None;
        let mut getter = None;
        let mut setter = None;
        let mut name = None;

        for attr in attrs {
            if attr.is_constructor() {
                constructor = Some(attr);
            } else if attr.is_multi_threaded() {
                multi_threaded = Some(attr);
            } else if attr.is_getter() {
                getter = Some(attr);
            } else if attr.is_setter() {
                setter = Some(attr);
            } else if let Some(name_lit) = attr.fn_name() {
                name = Some(name_lit.value());
            }
        }

        Self {
            constructor,
            multi_threaded,
            getter,
            setter,
            name,
        }
    }

    pub fn from_method_attribute(attribute: &Attribute) -> Result<Self> {
        //println!("token tree: {:#?}",attribute);

        match attribute.parse_meta()? {
            Meta::Path(_) => {
                // ignore node_bindgen which already know exists
                Ok(FunctionAttributes::default())
            }
            Meta::NameValue(n) => Err(Error::new(n.span(), "invalid attribute syntax")),
            Meta::List(list) => {
                let mut attrs = vec![];
                for nested_meta in list.nested.into_iter() {
                    match nested_meta {
                        NestedMeta::Meta(meta) => {
                            attrs.push(FunctionAttribute::from_ast(meta)?);
                        }
                        NestedMeta::Lit(lit) => {
                            return Err(Error::new(lit.span(), "unrecognized syntax"))
                        }
                    }
                }

                Ok(Self::from(attrs))
            }
        }
    }

    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    pub fn is_multi_threaded(&self) -> bool {
        self.multi_threaded.is_some()
    }

    pub fn is_constructor(&self) -> bool {
        self.constructor.is_some()
    }

    pub fn is_getter(&self) -> bool {
        self.getter.is_some()
    }

    pub fn is_setter(&self) -> bool {
        self.setter.is_some()
    }

    /// check if we method specific attribute if we not method
    pub fn valid_as_non_method(&self) -> Result<()> {
        /*
        if self.constructor.is_some() {
            return Err(Error::new(Span::call_site(), "constructor is only allowed in method"));
        }
        */

        if self.setter.is_some() {
            return Err(Error::new(
                Span::call_site(),
                "setter is only allowed in method",
            ));
        }

        if self.getter.is_some() {
            return Err(Error::new(
                Span::call_site(),
                "getter is only allowed in method",
            ));
        }

        Ok(())
    }
}
