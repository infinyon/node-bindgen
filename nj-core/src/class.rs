use std::ptr;

use log::debug;

use crate::sys::napi_value;
use crate::sys::napi_env;
use crate::sys::napi_callback_info;
use crate::sys::napi_ref;
use crate::val::JsEnv;
use crate::val::JsExports;
use crate::val::JsCallback;
use crate::NjError;
use crate::IntoJs;
use crate::PropertiesBuilder;


pub struct JSObjectWrapper<T> {
    wrapper: napi_ref,
    inner: T,
}

impl <T>JSObjectWrapper<T> {
    
    pub fn mut_inner(&mut self) -> &mut T {
        &mut self.inner
    }

    pub fn inner(&self) -> &T {
        &self.inner
    }
}

impl <T>JSObjectWrapper<T> where T: JSClass {
 
    /// wrap myself in the JS instance
    /// and saved the reference
    fn wrap(self, js_env: &JsEnv, js_cb: JsCallback) -> Result<napi_value,NjError> {

        let boxed_self = Box::new(self);
        let raw_ptr = Box::into_raw(boxed_self);    // rust no longer manages this struct

        let wrap =  js_env.wrap(js_cb.this(),raw_ptr as *mut u8,T::js_finalize)?;
    
        unsafe {
            // save the wrap reference in wrapper container
            let rust_ref: &mut Self = &mut * raw_ptr;
            rust_ref.wrapper = wrap;
        }

        // Finally, remove the reference in response to finalize callback
        // See footnote on `napi_wrap` documentation: https://nodejs.org/api/n-api.html#n_api_napi_wrap
        // 
        // "Caution: The optional returned reference (if obtained) should be deleted via napi_delete_reference 
        // ONLY in response to the finalize callback invocation. If it is deleted before then, 
        // then the finalize callback may never be invoked. Therefore, when obtaining a reference a 
        // finalize callback is also required in order to enable correct disposal of the reference."
        js_env.delete_reference(wrap)?;

        Ok(js_cb.this_owned())
    }
}


pub trait JSClass: Sized {

    const CLASS_NAME: &'static str;


    // create rust object from argument
    fn create_from_js(js_env: &JsEnv, cb: napi_callback_info) -> Result<(Self,JsCallback),NjError>;

    fn set_constructor(constructor: napi_ref);

    fn get_constructor() -> napi_ref;

    /// new instance
    fn new_instance(js_env: &JsEnv, js_args: Vec<napi_value>) -> Result<napi_value,NjError> {

        debug!("new instance with args: {:#?}",js_args);
        let constructor = js_env.get_reference_value(Self::get_constructor())?;
        js_env.new_instance(constructor, js_args)
    }

    /// given instance, return my object
    fn unwrap_mut(js_env: &JsEnv, instance: napi_value) -> Result<&'static mut Self,NjError> {
        Ok(js_env.unwrap_mut::<JSObjectWrapper<Self>>(instance)?.mut_inner())
    }

    fn unwrap(js_env: &JsEnv, instance: napi_value) -> Result<&'static Self,NjError> {
        Ok(js_env.unwrap::<JSObjectWrapper<Self>>(instance)?.inner())
    }

    fn properties() -> PropertiesBuilder {
        vec![].into()
    }


    /// define class and properties under exports
    fn js_init(js_exports: &mut JsExports) -> Result<(),NjError> {

        let js_constructor = js_exports.env().define_class(
            Self::CLASS_NAME,
            Self::js_new,
            Self::properties())?;
        
        // save the constructor reference, we need this later in order to instantiate
        let js_ref = js_exports.env().create_reference(js_constructor, 1)?;
        Self::set_constructor(js_ref);

        js_exports.set_name_property(Self::CLASS_NAME,js_constructor)?;
        Ok(())
    }

    
    
    /// call when Javascript class constructor is called
    /// For example:  new Car(...)
    extern "C" fn js_new(env: napi_env , info: napi_callback_info ) -> napi_value {

        let js_env = JsEnv::new(env);

        let result: Result<napi_value,NjError> = (|| {

            debug!("Class constructor called: {:#?}",std::any::type_name::<Self>());
           
            let target = js_env.get_new_target(info)?;
    
            if target.is_null() {
                
                Err(NjError::NoPlainConstructor)
            } else {
                debug!("invoked as constructor");  
    
                let (rust_obj,js_cb) = Self::create_from_js(&js_env,info)?;
                let my_obj = JSObjectWrapper {
                        inner: rust_obj,
                        wrapper: ptr::null_mut()
                    };
    
                my_obj.wrap(&js_env,js_cb)
            }
        })();

        result.to_js(&js_env)
       
    }

    /*
    /// convert my self as JS object
    fn as_js_instance(self,js_env: &JsEnv,js_args: Vec<napi_value>) -> napi_value {

        let new_instance = Self::new_instance(js_env,args);

        // unwrap the actual inner
    }
    */

    extern "C" fn js_finalize(_env: napi_env,finalize_data: *mut ::std::os::raw::c_void,
        _finalize_hint: *mut ::std::os::raw::c_void
    ) {

        debug!("my object finalize");
        unsafe {
            let ptr: *mut JSObjectWrapper<Self> = finalize_data as *mut JSObjectWrapper<Self>;
            Box::from_raw(ptr);
        }
        
    }

}