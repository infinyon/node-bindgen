use libc::size_t;

use crate::sys::napi_value;
use crate::val::JsEnv;
use crate::NjError;
use crate::napi_call_result;



/// convert to JS object
pub trait TryIntoJs {

    fn try_to_js(self,js_env: &JsEnv) -> Result<napi_value,NjError> ;    
    
}

impl TryIntoJs for bool {
    fn try_to_js(self, js_env: &JsEnv) -> Result<napi_value,NjError> {
        if self {
            js_env.create_int32(1)
        } else {
            js_env.create_int32(0)
        }
       
    }   
}

impl TryIntoJs for f64 {

    fn try_to_js(self, js_env: &JsEnv) -> Result<napi_value,NjError> {
        js_env.create_double(self)
    }
}

impl TryIntoJs for i64 {

    fn try_to_js(self, js_env: &JsEnv) -> Result<napi_value,NjError> {
        js_env.create_int64(self)
    }
}

impl TryIntoJs for i32 {

    fn try_to_js(self, js_env: &JsEnv) -> Result<napi_value,NjError> {
        js_env.create_int32(self)
    }
}


impl TryIntoJs for String {

    fn try_to_js(self, js_env: &JsEnv) -> Result<napi_value,NjError> {
        js_env.create_string_utf8(&self)
    }

}

impl TryIntoJs for () {
    fn try_to_js(self, _js_env: &JsEnv) -> Result<napi_value,NjError> {
        Ok(std::ptr::null_mut())
    }

}


impl <T,E>TryIntoJs for Result<T,E> where T: TryIntoJs, E: ToString {
    fn try_to_js(self, js_env: &JsEnv) -> Result<napi_value,NjError> {
        match self {
            Ok(val) => val.try_to_js(&js_env),
            Err(err) =>  Err(NjError::Other(err.to_string()))
        }
    }
}

impl TryIntoJs for napi_value {
    fn try_to_js(self,_js_env: &JsEnv) -> Result<napi_value,NjError> {
        Ok(self)
    }
}

/// convert to js including error
pub trait IntoJs {

    fn to_js(self,js_env: &JsEnv) -> napi_value;

}





pub trait JSValue: Sized {

    fn convert_to_rust(env: &JsEnv,js_value: napi_value) -> Result<Self,NjError>;
}

impl JSValue for f64 {

    fn convert_to_rust(env: &JsEnv,js_value: napi_value) -> Result<Self,NjError> {

        env.assert_type(js_value, crate::sys::napi_valuetype_napi_number)?;

        let mut value: f64 = 0.0;

        napi_call_result!(
            crate::sys::napi_get_value_double(env.inner(),js_value, &mut value)
        )?;

        Ok(value)
    }
}

impl JSValue for i32 {
    

    fn convert_to_rust(env: &JsEnv,js_value: napi_value) -> Result<Self,NjError> {

        env.assert_type(js_value, crate::sys::napi_valuetype_napi_number)?;

        let mut value: i32 = 0;

        napi_call_result!(
            crate::sys::napi_get_value_int32(env.inner(),js_value, &mut value)
        )?;

        Ok(value)
    }
}

impl JSValue for i64 {
    

    fn convert_to_rust(env: &JsEnv,js_value: napi_value) -> Result<Self,NjError> {

        env.assert_type(js_value, crate::sys::napi_valuetype_napi_number)?;

        let mut value: i64 = 0;

        napi_call_result!(
            crate::sys::napi_get_value_int64(env.inner(),js_value, &mut value)
        )?;

        Ok(value)
    }
}

impl JSValue for bool {

    fn convert_to_rust(env: &JsEnv,js_value: napi_value) -> Result<Self,NjError> {

        env.assert_type(js_value, crate::sys::napi_valuetype_napi_boolean)?;

        let mut value: bool = false;

        napi_call_result!(
            crate::sys::napi_get_value_bool(env.inner(),js_value, &mut value)
        )?;

        Ok(value)
    }
}



impl JSValue for String {


    fn convert_to_rust(env: &JsEnv,js_value: napi_value) -> Result<Self,NjError> {

        env.assert_type(js_value, crate::sys::napi_valuetype_napi_string)?;

        use crate::sys::napi_get_value_string_utf8;

        let mut chars: [u8; 1024] = [0;1024];
        let mut size: size_t = 0;

        napi_call_result!(
            napi_get_value_string_utf8(env.inner(),js_value,chars.as_mut_ptr() as *mut i8,1024,&mut size)
        )?;

        let my_chars: Vec<u8> = chars[0..size].into();

        String::from_utf8(my_chars).map_err(|err| err.into())
    }

}
