use syn::FnArg;
use syn::Ident;
use syn::Type;
use syn::Pat;
use syn::Error;
use syn::Generics;
use syn::TypeParam;
use syn::Signature;
use syn::TypeParamBound;
use syn::PathArguments;
use syn::Result;
use syn::spanned::Spanned;
use syn::ParenthesizedGenericArguments;
use syn::Receiver;

use super::MyTypePath;
use super::MyReferenceType;


/// Information about Function Arguments
#[derive(Debug,Default)]
pub struct FunctionArgs<'a> {
    pub args: Vec<FunctionArg<'a>>,
    pub is_method: bool,
    receiver: Option<&'a Receiver>,
}

impl <'a>FunctionArgs<'a> {

    pub fn from_ast(sig: &'a Signature) -> Result<Self> {

        //println!("fn: {:#?}",input_fn);
        let generics = &sig.generics;

        let mut args: Vec<FunctionArg> = vec![];

        let is_method = has_receiver(sig);

        // extract arguments,
        let i = 0;
        for ref arg in &sig.inputs {

           // println!("arg: {:#?}",arg);
            match arg {
                FnArg::Receiver(_) => {}
                FnArg::Typed(arg_type) => {
                 
                    match &*arg_type.pat {
                        Pat::Ident(identity) => {
                         
                            let arg = FunctionArg::new(
                                i,
                                &identity.ident,
                                &*arg_type.ty,
                                generics
                            )?;
                            args.push(arg);
                        },
                        _ => return Err(Error::new(arg_type.span(), "not supported type")),
                    }
                }
            }
        }

        Ok(Self {
            args,
            is_method,
            ..Default::default()
        })

    }
    

    pub fn inner(&self) -> &Vec<FunctionArg> {
        &self.args
    }

    pub fn len(&self) -> usize {
        self.args.len()
    }



}


/// find receiver if any, this will be used to indicate if this is method
fn has_receiver(sig: &Signature) -> bool {

    sig.inputs.iter().any(|input| {
        matches!(input, FnArg::Receiver(_rec))
    })
}


#[derive(Debug)]
pub struct FunctionArg<'a> {
    pub arg_index: u32,
    pub typ: FunctionArgType<'a>,
}

impl <'a> FunctionArg<'a>  {

    /// given this, convert into normalized type signature
    fn new(arg_index: u32, ident: &'a Ident, ty: &'a Type, generics: &'a Generics) -> Result<Self> {

        match ty {
            Type::Path(path_type) => {
                let my_type = MyTypePath::from(path_type)?;

                // check whether type references in the generic indicates this is closure
                if let Some(param) = find_generic(generics,my_type.ident()) {
                    let closure = ClosureType::from(ident,param)?;
                    Ok(Self {
                        arg_index,
                        typ: FunctionArgType::Closure(closure)
                    })
                } else {
                    Ok(Self {
                        arg_index,
                        typ: FunctionArgType::Path(my_type)
                    })
                }
            }
            Type::Reference(ref_type) => {
                let my_type = MyReferenceType::from(ref_type)?;
                /*
                if my_type.is_callback() {
                    Ok(Self {
                        arg_index,
                        typ: FunctionArgType::JSCallback(my_type)
                    })
                } else {
                    Ok(Self {
                        arg_index,
                        typ: FunctionArgType::Ref(my_type)
                    })
                }
                */
                Ok(Self {
                    arg_index,
                    typ: FunctionArgType::Ref(my_type)
                })
            }
            _ => Err(Error::new(ty.span(), "not supported type"))
        }
    }

    /*
    pub fn is_js_env(&self) -> bool {
        match self.typ {
            FunctionArgType::JsEnv(_) => true,
            _ => false,
        }
    }
    */
}


/// Categorize function argument
#[derive(Debug)]
pub enum FunctionArgType<'a> {
    Path(MyTypePath<'a>),           // normal type
    Ref(MyReferenceType<'a>),       // reference type
    Closure(ClosureType<'a>),       // closure callback
   // JsEnv(MyReferenceType<'a>),     // indicating that we want to receive JsEnv
}


/// find generic with match ident
fn find_generic<'a,'b>(generics: &'a Generics, ident: Option<&'b Ident>) -> Option<&'a TypeParam> {

    if let Some(ident) = ident {
        generics.type_params().find(|ty| *ty.ident.to_string() == *ident.to_string())
    } else {
        None
    }

}


#[derive(Debug)]
pub struct ClosureType<'a> {
    //pub ty: &'a ParenthesizedGenericArguments,
    pub inputs: Vec<MyTypePath<'a>>,
    pub ident: &'a Ident
}

impl <'a>ClosureType<'a> {
    // try to see if we can find closure, otherwise return none
    pub fn from(ident: &'a Ident,param: &'a TypeParam) -> Result<Self> {
        for ref bound in &param.bounds {
            match bound {
                TypeParamBound::Trait(tt) => {
                    for ref segment in &tt.path.segments {
                        match segment.arguments {
                            
                            PathArguments::Parenthesized(ref path) => return Ok(Self {
                                ident,
                                inputs: find_inputs(path)?
                            }),
                            _ => return Err(Error::new(param.span(), "not supported closure type")),
                        }
                    }
                    return Err(Error::new(param.span(), "not supported closure type"))
                }
                TypeParamBound::Lifetime(_) => return Err(Error::new(param.span(), "not supported closure type")),
            }
        }
        Err(Error::new(param.span(), "not supported closure type"))
    }

    
    // name of function is used by thread safe function to complete closure
    pub fn async_js_callback_identifier(&self) -> Ident {

        use proc_macro2::Span;

        Ident::new(&format!("thread_safe_{}_complete",self.ident),Span::call_site())
    }
    

}


fn find_inputs(ty: &ParenthesizedGenericArguments)  -> Result<Vec<MyTypePath>> {

    let mut types: Vec<MyTypePath> = vec![];

    for path in &ty.inputs  {
        let my_type = match path {
            Type::Path(ref path_type) =>  {
                match MyTypePath::from(path_type) {
                    Ok(m_type) => m_type,
                    Err(err) => return Err(err)
                }
            },
            _ => return Err(Error::new(ty.span(), "not supported closure type"))
        };
        types.push(my_type);
    }

    Ok(types)

}