use std::ptr;

use crate::TryIntoJs;
use crate::sys::napi_value;
use crate::val::JsEnv;
use crate::NjError;

/// Rust representation of Nodejs [ArrayBuffer](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer)
/// In regular cases can be used ArrayBuffer and it will be more effective. But in the case of usage node_bindgen in electron context,
/// ArrayBuffer will not work because of restrictions from electron side. SafeArrayBuffer can be used as an effective workaround. Theoretically,
/// usage of SafeArrayBuffer might give a very small performance degradation compared to ArrayBuffer, but it can be ignored in most cases.
///
/// # Examples
///
/// ```no_run
/// use node_bindgen::derive::node_bindgen;
/// use node_bindgen::core::{buffer::JSArrayBuffer, safebuffer::{SafeArrayBuffer}};
///
/// #[node_bindgen]
/// fn msg_from_js(data: JSArrayBuffer) -> Result<String, NjError> {
///   let message = String::from_utf8(data.to_vec())?;
///    Ok(format!("reply {}", message))
/// }
/// #[node_bindgen]
/// fn msg_to_js() -> Result<SafeArrayBuffer, NjError> {
///   let message = String::from("Hello from Rust!");
///   Ok(SafeArrayBuffer::new(message.to_vec()))
/// }
/// ```
pub struct SafeArrayBuffer {
    data: Vec<u8>,
}

use std::fmt;
use std::fmt::Debug;

impl Debug for SafeArrayBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("SafeArrayBuffer len: {}", self.data.len()))
    }
}

impl SafeArrayBuffer {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }
}

impl TryIntoJs for SafeArrayBuffer {
    fn try_to_js(self, js_env: &JsEnv) -> Result<napi_value, NjError> {
        let len = self.data.len();
        let mut napi_buffer = ptr::null_mut();
        crate::napi_call_result!(crate::sys::napi_create_buffer_copy(
            js_env.inner(),
            len,
            self.data.as_ptr() as *const std::ffi::c_void,
            std::ptr::null_mut(),
            &mut napi_buffer,
        ))?;
        Ok(napi_buffer)
    }
}
