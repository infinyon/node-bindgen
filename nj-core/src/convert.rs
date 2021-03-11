use libc::size_t;
use std::ptr;

use crate::sys::napi_value;
use crate::val::JsEnv;
use crate::NjError;
use crate::napi_call_result;

/// convert to JS object
pub trait TryIntoJs {
    fn try_to_js(self, js_env: &JsEnv) -> Result<napi_value, NjError>;
}

impl TryIntoJs for bool {
    fn try_to_js(self, js_env: &JsEnv) -> Result<napi_value, NjError> {
        js_env.create_boolean(self)
    }
}

impl TryIntoJs for f64 {
    fn try_to_js(self, js_env: &JsEnv) -> Result<napi_value, NjError> {
        js_env.create_double(self)
    }
}

impl TryIntoJs for i64 {
    fn try_to_js(self, js_env: &JsEnv) -> Result<napi_value, NjError> {
        js_env.create_int64(self)
    }
}

impl TryIntoJs for i32 {
    fn try_to_js(self, js_env: &JsEnv) -> Result<napi_value, NjError> {
        js_env.create_int32(self)
    }
}
impl TryIntoJs for u64 {
    fn try_to_js(self, js_env: &JsEnv) -> Result<napi_value, NjError> {
        js_env.create_bigint_uint64(self)
    }
}

impl TryIntoJs for String {
    fn try_to_js(self, js_env: &JsEnv) -> Result<napi_value, NjError> {
        js_env.create_string_utf8(&self)
    }
}

impl TryIntoJs for () {
    fn try_to_js(self, js_env: &JsEnv) -> Result<napi_value, NjError> {
        js_env.get_undefined()
    }
}

impl<T, E> TryIntoJs for Result<T, E>
where
    T: TryIntoJs,
    E: ToString,
{
    fn try_to_js(self, js_env: &JsEnv) -> Result<napi_value, NjError> {
        match self {
            Ok(val) => val.try_to_js(&js_env),
            Err(err) => Err(NjError::Other(err.to_string())),
        }
    }
}

impl<T> TryIntoJs for Option<T>
where
    T: TryIntoJs,
{
    fn try_to_js(self, js_env: &JsEnv) -> Result<napi_value, NjError> {
        match self {
            Some(val) => val.try_to_js(&js_env),
            None => js_env.get_null(),
        }
    }
}

impl TryIntoJs for napi_value {
    fn try_to_js(self, _js_env: &JsEnv) -> Result<napi_value, NjError> {
        Ok(self)
    }
}

impl<T> TryIntoJs for Vec<T>
where
    T: TryIntoJs,
{
    fn try_to_js(self, js_env: &JsEnv) -> Result<napi_value, NjError> {
        let array = js_env.create_array_with_len(self.len())?;
        for (i, element) in self.into_iter().enumerate() {
            let js_element = element.try_to_js(js_env)?;
            js_env.set_element(array, js_element, i)?;
        }

        Ok(array)
    }
}

/// convert to js including error
pub trait IntoJs {
    fn into_js(self, js_env: &JsEnv) -> napi_value;
}

/// Convert napi value to Rust value
///
pub trait JSValue<'a>: Sized {
    fn label() -> &'static str {
        std::any::type_name::<Self>()
    }

    fn convert_to_rust(env: &'a JsEnv, js_value: napi_value) -> Result<Self, NjError>;
}

impl JSValue<'_> for f64 {
    fn convert_to_rust(env: &JsEnv, js_value: napi_value) -> Result<Self, NjError> {
        env.assert_type(js_value, crate::sys::napi_valuetype_napi_number)?;

        let mut value: f64 = 0.0;

        napi_call_result!(crate::sys::napi_get_value_double(
            env.inner(),
            js_value,
            &mut value
        ))?;

        Ok(value)
    }
}

impl JSValue<'_> for i32 {
    fn convert_to_rust(env: &JsEnv, js_value: napi_value) -> Result<Self, NjError> {
        env.assert_type(js_value, crate::sys::napi_valuetype_napi_number)?;

        let mut value: i32 = 0;

        napi_call_result!(crate::sys::napi_get_value_int32(
            env.inner(),
            js_value,
            &mut value
        ))?;

        Ok(value)
    }
}

impl JSValue<'_> for u32 {
    fn convert_to_rust(env: &JsEnv, js_value: napi_value) -> Result<Self, NjError> {
        env.assert_type(js_value, crate::sys::napi_valuetype_napi_number)?;

        let mut value: u32 = 0;

        napi_call_result!(crate::sys::napi_get_value_uint32(
            env.inner(),
            js_value,
            &mut value
        ))?;

        Ok(value)
    }
}

impl JSValue<'_> for i64 {
    fn convert_to_rust(env: &JsEnv, js_value: napi_value) -> Result<Self, NjError> {
        env.assert_type(js_value, crate::sys::napi_valuetype_napi_number)?;

        let mut value: i64 = 0;

        napi_call_result!(crate::sys::napi_get_value_int64(
            env.inner(),
            js_value,
            &mut value
        ))?;

        Ok(value)
    }
}

impl JSValue<'_> for bool {
    fn convert_to_rust(env: &JsEnv, js_value: napi_value) -> Result<Self, NjError> {
        env.assert_type(js_value, crate::sys::napi_valuetype_napi_boolean)?;

        let mut value: bool = false;

        napi_call_result!(crate::sys::napi_get_value_bool(
            env.inner(),
            js_value,
            &mut value
        ))?;

        Ok(value)
    }
}

impl JSValue<'_> for String {
    fn convert_to_rust(env: &JsEnv, js_value: napi_value) -> Result<Self, NjError> {
        env.assert_type(js_value, crate::sys::napi_valuetype_napi_string)?;

        use crate::sys::napi_get_value_string_utf8;

        let mut string_size: size_t = 0;

        napi_call_result!(napi_get_value_string_utf8(
            env.inner(),
            js_value,
            ptr::null_mut(),
            0,
            &mut string_size
        ))?;

        string_size += 1;

        let chars_vec: Vec<u8> = vec![0; string_size];
        let mut chars: Box<[u8]> = chars_vec.into_boxed_slice();
        let mut read_size: size_t = 0;

        napi_call_result!(napi_get_value_string_utf8(
            env.inner(),
            js_value,
            chars.as_mut_ptr() as *mut ::std::os::raw::c_char,
            string_size,
            &mut read_size
        ))?;

        let my_chars: Vec<u8> = chars[0..read_size].into();

        String::from_utf8(my_chars).map_err(|err| err.into())
    }
}

impl<'a> JSValue<'a> for &'a str {
    fn convert_to_rust(env: &'a JsEnv, js_value: napi_value) -> Result<Self, NjError> {
        use crate::sys::napi_get_buffer_info;

        let mut len: size_t = 0;
        let mut data = ptr::null_mut();

        napi_call_result!(napi_get_buffer_info(
            env.inner(),
            js_value,
            &mut data,
            &mut len
        ))?;

        unsafe {
            let i8slice = std::slice::from_raw_parts(data as *mut ::std::os::raw::c_char, len);
            let u8slice = &*(i8slice as *const _ as *const [u8]);
            std::str::from_utf8(u8slice).map_err(|err| err.into())
        }
    }
}

impl<'a, T> JSValue<'a> for Vec<T>
where
    T: JSValue<'a>,
{
    fn convert_to_rust(env: &'a JsEnv, js_value: napi_value) -> Result<Self, NjError> {
        if !env.is_array(js_value)? {
            return Err(NjError::Other("not array".to_owned()));
        }

        use crate::sys::napi_get_array_length;

        let mut length: u32 = 0;

        napi_call_result!(napi_get_array_length(env.inner(), js_value, &mut length))?;

        let mut elements = vec![];

        for i in 0..length {
            let js_element = env.get_element(js_value, i)?;
            elements.push(T::convert_to_rust(env, js_element)?);
        }

        Ok(elements)
    }
}
