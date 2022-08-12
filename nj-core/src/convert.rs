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

impl TryIntoJs for i8 {
    fn try_to_js(self, js_env: &JsEnv) -> Result<napi_value, NjError> {
        js_env.create_int32(self as i32)
    }
}

impl TryIntoJs for i16 {
    fn try_to_js(self, js_env: &JsEnv) -> Result<napi_value, NjError> {
        js_env.create_int32(self as i32)
    }
}

impl TryIntoJs for i32 {
    fn try_to_js(self, js_env: &JsEnv) -> Result<napi_value, NjError> {
        js_env.create_int32(self)
    }
}

impl TryIntoJs for i64 {
    fn try_to_js(self, js_env: &JsEnv) -> Result<napi_value, NjError> {
        js_env.create_int64(self)
    }
}

impl TryIntoJs for u8 {
    fn try_to_js(self, js_env: &JsEnv) -> Result<napi_value, NjError> {
        js_env.create_uint32(self as u32)
    }
}

impl TryIntoJs for u16 {
    fn try_to_js(self, js_env: &JsEnv) -> Result<napi_value, NjError> {
        js_env.create_uint32(self as u32)
    }
}

impl TryIntoJs for u32 {
    fn try_to_js(self, js_env: &JsEnv) -> Result<napi_value, NjError> {
        js_env.create_uint32(self)
    }
}

impl TryIntoJs for u64 {
    fn try_to_js(self, js_env: &JsEnv) -> Result<napi_value, NjError> {
        js_env.create_bigint_uint64(self)
    }
}

impl TryIntoJs for usize {
    fn try_to_js(self, js_env: &JsEnv) -> Result<napi_value, NjError> {
        js_env.create_bigint_uint64(self as u64)
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

impl TryIntoJs for NjError {
    fn try_to_js(self, _js_env: &JsEnv) -> Result<napi_value, NjError> {
        // Re-throw the error into JS
        Err(self)
    }
}

impl TryIntoJs for std::io::Error {
    fn try_to_js(self, _js_env: &JsEnv) -> Result<napi_value, NjError> {
        let message = self.to_string();
        Err(NjError::Other(message))
    }
}

#[cfg(feature = "serde_json")]
impl TryIntoJs for serde_json::Value {
    fn try_to_js(self, js_env: &JsEnv) -> Result<napi_value, NjError> {
        match self {
            serde_json::Value::Null => js_env.get_null(),
            serde_json::Value::Bool(val) => val.try_to_js(js_env),
            serde_json::Value::Number(num) => {
                if num.is_i64() {
                    js_env.create_int64(num.as_i64().unwrap())
                } else if num.is_u64() {
                    js_env.create_bigint_uint64(num.as_u64().unwrap())
                } else {
                    js_env.create_double(num.as_f64().unwrap())
                }
            }
            serde_json::Value::String(string) => string.try_to_js(js_env),
            serde_json::Value::Array(arr) => arr.try_to_js(js_env),
            serde_json::Value::Object(obj) => obj.try_to_js(js_env),
        }
    }
}

#[cfg(feature = "convert-uuid")]
impl TryIntoJs for uuid::Uuid {
    fn try_to_js(self, js_env: &JsEnv) -> Result<napi_value, NjError> {
        let as_str = self
            .as_hyphenated()
            .encode_lower(&mut uuid::Uuid::encode_buffer())
            .to_string();

        as_str.try_to_js(js_env)
    }
}

#[cfg(feature = "convert-uuid")]
impl JSValue<'_> for uuid::Uuid {
    fn convert_to_rust(env: &JsEnv, js_value: napi_value) -> Result<Self, NjError> {
        let string = String::convert_to_rust(env, js_value)?;
        let uuid = uuid::Uuid::parse_str(&string)
            .map_err(|e| NjError::Other(format!("Failed to parse Uuid: {}", e)))?;
        Ok(uuid)
    }
}

impl<T, E> TryIntoJs for Result<T, E>
where
    T: TryIntoJs,
    E: TryIntoJs,
{
    fn try_to_js(self, js_env: &JsEnv) -> Result<napi_value, NjError> {
        match self {
            Ok(val) => val.try_to_js(js_env),
            Err(err) => Err(NjError::Native(err.try_to_js(js_env)?)),
        }
    }
}

impl<T> TryIntoJs for Option<T>
where
    T: TryIntoJs,
{
    fn try_to_js(self, js_env: &JsEnv) -> Result<napi_value, NjError> {
        match self {
            Some(val) => val.try_to_js(js_env),
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

impl<T> TryIntoJs for std::collections::HashMap<String, T>
where
    T: TryIntoJs,
{
    fn try_to_js(self, js_env: &JsEnv) -> Result<napi_value, NjError> {
        let obj = js_env.create_object()?;
        for (key, value) in self {
            let js_value = value.try_to_js(js_env)?;
            js_env.set_property(obj, &key, js_value)?;
        }

        Ok(obj)
    }
}

#[cfg(feature = "serde_json")]
impl TryIntoJs for serde_json::map::Map<String, serde_json::Value> {
    fn try_to_js(self, js_env: &JsEnv) -> Result<napi_value, NjError> {
        use crate::val::JsObject;
        let mut obj = JsObject::new(*js_env, js_env.create_object()?);

        let converted_obj = self
            .into_iter()
            .map(|(key, val)| val.try_to_js(js_env).map(|v| (key, v)))
            .collect::<Result<Vec<(String, napi_value)>, NjError>>()?;

        for (key, val) in converted_obj {
            obj.set_property(&key, val)?;
        }

        Ok(obj.napi_value())
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
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
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
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
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
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
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
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
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
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
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
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
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
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
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
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn convert_to_rust(env: &'a JsEnv, js_value: napi_value) -> Result<Self, NjError> {
        if !env.is_array(js_value)? {
            return Err(NjError::Other(
                "Provided value was not an array as expected".to_owned(),
            ));
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

macro_rules! impl_js_value_for_tuple {
    ( $( $len:expr => ( $( $n:tt $t:ident ),+ $(,)? ))+ ) => {
        $(
            impl<'a $(, $t)+ > crate::JSValue<'a> for ($($t,)+)
            where
                $($t: JSValue<'a> + Send,)+
            {
                #[allow(clippy::not_unsafe_ptr_arg_deref)]
                fn convert_to_rust(env: &'a JsEnv, js_value: napi_value) -> Result<Self, NjError> {
                    use crate::sys::napi_get_array_length;
                    if !env.is_array(js_value)? {
                        return Err(NjError::Other("Tuples must come from JS arrays".to_owned()));
                    }

                    let mut length: u32 = 0;
                    napi_call_result!(napi_get_array_length(env.inner(), js_value, &mut length))?;
                    let required_length = $len;
                    if length != required_length {
                        return Err(NjError::Other(format!("{n}Tuple must have exactly length {n}", n = required_length)));
                    }

                    $(
                        let js_element = env.get_element(js_value, $n)?;
                        #[allow(non_snake_case)]
                        let $t = $t::convert_to_rust(env, js_element)?;
                    )+

                    Ok(( $($t,)+ ))
                }
            }
        )+
    }
}

impl_js_value_for_tuple! {
    1 => (0 T0)
    2 => (0 T0, 1 T1)
    3 => (0 T0, 1 T1, 2 T2)
    4 => (0 T0, 1 T1, 2 T2, 3 T3)
    5 => (0 T0, 1 T1, 2 T2, 3 T3, 4 T4)
    6 => (0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5)
    7 => (0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6)
    8 => (0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7)
    9 => (0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8)
}

macro_rules! impl_try_into_js_for_tuple {
    ( $( $len:expr => ( $( $n:tt $t:tt ),+ $(,)? ))+ ) => {
        $(
            impl<$( $t ),+> crate::TryIntoJs for ( $( $t, )+ )
                where $( $t: TryIntoJs + Send, )+
            {
                fn try_to_js(self, js_env: &JsEnv) -> Result<napi_value, NjError> {
                    let length: usize = $len;
                    let array = js_env.create_array_with_len(length)?;

                    #[allow(non_snake_case)]
                    let ( $($t, )+ ) = self;

                    $(
                        let js_element = $t.try_to_js(js_env)?;
                        js_env.set_element(array, js_element, $n)?;
                    )+

                    Ok(array)
                }
            }
        )+
    }
}

impl_try_into_js_for_tuple! {
    1 => (0 T0)
    2 => (0 T0, 1 T1)
    3 => (0 T0, 1 T1, 2 T2)
    4 => (0 T0, 1 T1, 2 T2, 3 T3)
    5 => (0 T0, 1 T1, 2 T2, 3 T3, 4 T4)
    6 => (0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5)
    7 => (0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6)
    8 => (0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7)
    9 => (0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8)
}
