
use proc_macro2::Span;
use syn::Ident;
use syn::FnArg;
use syn::Signature;
use syn::Receiver;
use syn::LitStr;
use syn::ReturnType;
use quote::quote;
use proc_macro2::TokenStream;

use crate::ast::FunctionArgs;
use crate::ast::FunctionAttributes;
use crate::util::ident;

///  Context for code function code generation
pub struct FnGeneratorCtx<'a> {
    pub args: &'a FunctionArgs<'a>,
    pub attributes: &'a FunctionAttributes,
    pub sig: &'a Signature,
    receiver: Option<&'a Receiver>
}

impl <'a>FnGeneratorCtx<'a> {

    
    // TODO: is_multi_threaded should be check and return self
    pub fn new(
            sig: &'a Signature,
            args: &'a FunctionArgs<'a>,
            attributes: &'a FunctionAttributes,
    ) -> Self {
        Self {
            sig,
            args,
            attributes,
            receiver: find_receiver(sig)
        }
    }


    /// function name identifier
    pub fn fn_name(&self) -> &Ident {
        &self.sig.ident
    }

    pub fn is_method(&self) -> bool {
        self.receiver.is_some()
    }


    pub fn is_async(&self) -> bool {
        self.sig.asyncness.is_some()
    }

    /// check whether this function return ()
    pub fn has_default_output(&self) -> bool {
        matches!(self.sig.output, ReturnType::Default)
    }

    

    pub fn napi_fn_id(&self) -> Ident {
        ident(&format!("napi_{}", self.fn_name()))
    }

    pub fn attributes(&self) -> &FunctionAttributes {
        &self.attributes
    }

    /// used for registering in the Napi
    pub fn property_name(&self) -> LitStr {

        use crate::util::default_function_property_name;

        if let Some(name) = self.attributes().name() {
            LitStr::new(name, Span::call_site())
        } else {
            LitStr::new(&default_function_property_name(&self.fn_name().to_string()),Span::call_site())
        }
    
    }

    // callback type name
    pub fn callback_type_name(&self) -> TokenStream {
            
        let callback_type = if self.attributes.is_multi_threaded() {
            "JsMultiThreadedCallbackFunction"
        } else {
            "JsCallbackFunction"
        };

        let ident = Ident::new(callback_type, Span::call_site());
        quote! { #ident }
    }

    
}




fn find_receiver(sig: &Signature) -> Option<&Receiver> {

    sig.inputs.iter().find_map( |arg| 
    
        match arg {
            FnArg::Receiver(rec) => Some(rec),
            _ => None
        }
    )

}