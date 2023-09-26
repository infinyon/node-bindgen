use std::ptr;
use std::ffi::CString;
use std::collections::VecDeque;

use tracing::instrument;
use tracing::{error, debug, trace};

use crate::sys::napi_env;
use crate::sys::napi_value;
use crate::sys::napi_callback_info;
use crate::sys::napi_callback_raw;
use crate::sys::napi_finalize_raw;
use crate::sys::napi_valuetype;
use crate::sys::napi_get_property;
use crate::sys::napi_has_property;
use crate::sys::napi_ref;
use crate::sys::napi_deferred;
use crate::sys::napi_threadsafe_function_call_js;

use crate::napi_call_result;
use crate::napi_call_assert;
use crate::PropertiesBuilder;
use crate::NjError;
use crate::JSObjectWrapper;
use crate::JSValue;
use crate::TryIntoJs;

fn napi_value_type_to_string(js_type: napi_valuetype) -> &'static str {
    match js_type {
        crate::sys::napi_valuetype_napi_bigint => "big_int",
        crate::sys::napi_valuetype_napi_boolean => "bool",
        crate::sys::napi_valuetype_napi_number => "number",
        crate::sys::napi_valuetype_napi_string => "string",
        crate::sys::napi_valuetype_napi_symbol => "symbol",
        crate::sys::napi_valuetype_napi_function => "function",
        crate::sys::napi_valuetype_napi_null => "null",
        crate::sys::napi_valuetype_napi_external => "external",
        crate::sys::napi_valuetype_napi_undefined => "undefined",
        _ => "other",
    }
}

#[derive(Clone)]
pub struct JsNapiValue(napi_value);

unsafe impl Send for JsNapiValue {}

impl From<napi_value> for JsNapiValue {
    fn from(value: napi_value) -> Self {
        Self(value)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct JsEnv(napi_env);

impl From<napi_env> for JsEnv {
    fn from(env: napi_env) -> Self {
        Self(env)
    }
}

unsafe impl Send for JsEnv {}
unsafe impl Sync for JsEnv {}

impl JsEnv {
    pub fn new(env: napi_env) -> Self {
        Self(env)
    }

    pub fn inner(&self) -> napi_env {
        self.0
    }

    pub fn create_string_utf8(&self, r_string: &str) -> Result<napi_value, NjError> {
        trace!("create utf8 string: {}", r_string);
        use nj_sys::napi_create_string_utf8;

        let mut js_value = ptr::null_mut();
        napi_call_result!(napi_create_string_utf8(
            self.0,
            r_string.as_ptr() as *const ::std::os::raw::c_char,
            r_string.len(),
            &mut js_value
        ))?;
        Ok(js_value)
    }

    pub fn create_string_utf8_from_bytes(&self, r_string: &[u8]) -> Result<napi_value, NjError> {
        use nj_sys::napi_create_string_utf8;

        let mut js_value = ptr::null_mut();
        napi_call_result!(napi_create_string_utf8(
            self.0,
            r_string.as_ptr() as *const ::std::os::raw::c_char,
            r_string.len(),
            &mut js_value
        ))?;
        Ok(js_value)
    }

    pub fn create_double(&self, value: f64) -> Result<napi_value, NjError> {
        let mut result = ptr::null_mut();
        napi_call_result!(crate::sys::napi_create_double(self.0, value, &mut result))?;
        Ok(result)
    }

    pub fn create_int64(&self, value: i64) -> Result<napi_value, NjError> {
        let mut result = ptr::null_mut();
        napi_call_result!(crate::sys::napi_create_int64(self.0, value, &mut result))?;
        Ok(result)
    }

    pub fn create_int32(&self, value: i32) -> Result<napi_value, NjError> {
        let mut result = ptr::null_mut();
        napi_call_result!(crate::sys::napi_create_int32(self.0, value, &mut result))?;
        Ok(result)
    }

    pub fn create_uint32(&self, value: u32) -> Result<napi_value, NjError> {
        let mut result = ptr::null_mut();
        napi_call_result!(crate::sys::napi_create_uint32(self.0, value, &mut result))?;
        Ok(result)
    }

    pub fn create_bigint_uint64(&self, value: u64) -> Result<napi_value, NjError> {
        let mut result = ptr::null_mut();
        napi_call_result!(crate::sys::napi_create_bigint_uint64(
            self.0,
            value,
            &mut result
        ))?;
        Ok(result)
    }

    pub fn create_object(&self) -> Result<napi_value, NjError> {
        let mut result = ptr::null_mut();

        napi_call_result!(crate::sys::napi_create_object(self.0, &mut result))?;
        Ok(result)
    }

    pub fn create_boolean(&self, value: bool) -> Result<napi_value, NjError> {
        let mut result = ptr::null_mut();
        napi_call_result!(crate::sys::napi_get_boolean(self.0, value, &mut result,))?;
        Ok(result)
    }

    pub fn create_array_with_len(&self, len: usize) -> Result<napi_value, NjError> {
        let mut array = ptr::null_mut();

        napi_call_result!(crate::sys::napi_create_array_with_length(
            self.0, len, &mut array
        ))?;
        Ok(array)
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn set_element(
        &self,
        object: napi_value,
        element: napi_value,
        index: usize,
    ) -> Result<(), NjError> {
        napi_call_result!(crate::sys::napi_set_element(
            self.0,
            object,
            index as u32,
            element
        ))?;
        Ok(())
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn get_element(&self, array: napi_value, index: u32) -> Result<napi_value, NjError> {
        let mut element = ptr::null_mut();

        napi_call_result!(crate::sys::napi_get_element(
            self.0,
            array,
            index,
            &mut element
        ))?;
        Ok(element)
    }

    /// check if napi value is array
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn is_array(&self, array: napi_value) -> Result<bool, NjError> {
        let mut result: bool = false;

        napi_call_result!(crate::sys::napi_is_array(self.0, array, &mut result))?;

        Ok(result)
    }

    /// check if napi value is array buffer
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn is_array_buffer(&self, array: napi_value) -> Result<bool, NjError> {
        let mut result: bool = false;

        napi_call_result!(crate::sys::napi_is_arraybuffer(self.0, array, &mut result))?;

        Ok(result)
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn is_buffer(&self, n_value: napi_value) -> Result<bool, NjError> {
        let mut result: bool = false;

        napi_call_result!(crate::sys::napi_is_buffer(self.0, n_value, &mut result))?;

        Ok(result)
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn is_date(&self, n_value: napi_value) -> Result<bool, NjError> {
        let mut result: bool = false;

        napi_call_result!(crate::sys::napi_is_date(self.0, n_value, &mut result))?;

        Ok(result)
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn is_error(&self, n_value: napi_value) -> Result<bool, NjError> {
        let mut result: bool = false;

        napi_call_result!(crate::sys::napi_is_error(self.0, n_value, &mut result))?;

        Ok(result)
    }

    pub fn get_global(&self) -> Result<napi_value, NjError> {
        use nj_sys::napi_get_global;

        let mut js_global = ptr::null_mut();
        napi_call_result!(napi_get_global(self.0, &mut js_global))?;
        Ok(js_global)
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn call_function(
        &self,
        recv: napi_value,
        func: napi_value,
        mut argv: Vec<napi_value>,
    ) -> Result<napi_value, NjError> {
        use nj_sys::napi_call_function;

        let mut result = ptr::null_mut();

        napi_call_result!(napi_call_function(
            self.0,
            recv,
            func,
            argv.len(),
            argv.as_mut_ptr(),
            &mut result
        ))?;

        Ok(result)
    }

    /// get callback information
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    #[instrument]
    pub fn get_cb_info(
        &self,
        info: napi_callback_info,
        max_count: usize,
    ) -> Result<JsCallback, NjError> {
        use nj_sys::napi_get_cb_info;

        let mut this = ptr::null_mut();

        let mut argc = max_count;
        let mut args = vec![ptr::null_mut(); max_count];
        napi_call_result!(napi_get_cb_info(
            self.0,
            info,
            &mut argc,
            args.as_mut_ptr(),
            &mut this,
            ptr::null_mut()
        ))?;

        trace!(argc, "actual argc");
        // truncate arg to actual received count
        args.resize(argc, ptr::null_mut());

        Ok(JsCallback::new(JsEnv::new(self.0), this, args))
    }

    /// define classes
    #[instrument(skip(properties))]
    pub fn define_class(
        &self,
        name: &str,
        constructor: napi_callback_raw,
        properties: PropertiesBuilder,
    ) -> Result<napi_value, NjError> {
        debug!(?properties, "defining class",);
        let mut js_constructor = ptr::null_mut();
        let mut raw_properties = properties.as_raw_properties();
        napi_call_result!(crate::sys::napi_define_class(
            self.0,
            name.as_ptr() as *const ::std::os::raw::c_char,
            name.len(),
            Some(constructor),
            ptr::null_mut(),
            raw_properties.len(),
            raw_properties.as_mut_ptr(),
            &mut js_constructor
        ))?;

        Ok(js_constructor)
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn create_reference(&self, cons: napi_value, count: u32) -> Result<napi_ref, NjError> {
        let mut result = ptr::null_mut();
        napi_call_result!(crate::sys::napi_create_reference(
            self.0,
            cons,
            count,
            &mut result
        ))?;

        Ok(result)
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn delete_reference(&self, ref_: napi_ref) -> Result<(), NjError> {
        Ok(napi_call_result!(crate::sys::napi_delete_reference(
            self.0, ref_
        ))?)
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    #[instrument]
    pub fn get_new_target(&self, info: napi_callback_info) -> Result<napi_value, NjError> {
        let mut result = ptr::null_mut();
        napi_call_result!(crate::sys::napi_get_new_target(self.0, info, &mut result))?;
        debug!(?result, "got new target");
        Ok(result)
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    #[instrument]
    pub fn wrap(
        &self,
        js_object: napi_value,
        rust_obj: *mut u8,
        finalize: napi_finalize_raw,
    ) -> Result<napi_ref, NjError> {
        let mut result = ptr::null_mut();

        debug!("napi wrap");
        napi_call_result!(crate::sys::napi_wrap(
            self.0,
            js_object,
            rust_obj as *mut core::ffi::c_void,
            Some(finalize),
            ptr::null_mut(),
            &mut result
        ))?;

        Ok(result)
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    #[instrument]
    pub fn unwrap<T>(&self, js_this: napi_value) -> Result<&'static T, NjError> {
        let mut result: *mut ::std::os::raw::c_void = ptr::null_mut();
        napi_call_result!(crate::sys::napi_unwrap(self.0, js_this, &mut result))?;

        Ok(unsafe {
            debug!(?result, "got back raw pointer");
            if result == ptr::null_mut() {
                return Err(NjError::Other("unwrap got null pointer".to_string()));
            }
            let rust_ref: &T = &mut *(result as *mut T);
            rust_ref
        })
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    #[instrument]
    pub fn unwrap_mut<T>(&self, js_this: napi_value) -> Result<&'static mut T, NjError> {
        let mut result: *mut ::std::os::raw::c_void = ptr::null_mut();
        debug!(env = ?self.0,"napi unwrap");
        napi_call_result!(crate::sys::napi_unwrap(self.0, js_this, &mut result))?;
        Ok(unsafe {
            debug!(?result, "got back raw pointer");
            if result == ptr::null_mut() {
                return Err(NjError::Other("unwrap mut null pointer".to_string()));
            }
            let ptr = result as *mut T;
            let rust_ref: &mut T = &mut *(ptr);
            rust_ref
        })
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn new_instance(
        &self,
        constructor: napi_value,
        mut args: Vec<napi_value>,
    ) -> Result<napi_value, NjError> {
        trace!(args = args.len(), "napi new instance");
        let mut result = ptr::null_mut();
        napi_call_result!(crate::sys::napi_new_instance(
            self.0,
            constructor,
            args.len(),
            args.as_mut_ptr(),
            &mut result
        ))?;

        Ok(result)
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn get_reference_value(&self, obj_ref: napi_ref) -> Result<napi_value, NjError> {
        let mut result = ptr::null_mut();
        napi_call_result!(crate::sys::napi_get_reference_value(
            self.0,
            obj_ref,
            &mut result
        ))?;

        Ok(result)
    }

    /// create promise and deferred
    pub fn create_promise(&self) -> Result<(napi_value, napi_deferred), NjError> {
        let mut deferred = ptr::null_mut();
        let mut promise = ptr::null_mut();

        napi_call_result!(crate::sys::napi_create_promise(
            self.0,
            &mut deferred,
            &mut promise
        ))?;

        Ok((promise, deferred))
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn resolve_deferred(
        &self,
        deferred: napi_deferred,
        resolution: napi_value,
    ) -> Result<(), NjError> {
        napi_call_result!(crate::sys::napi_resolve_deferred(
            self.0, deferred, resolution
        ))
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn reject_deferred(
        &self,
        deferred: napi_deferred,
        rejection: napi_value,
    ) -> Result<(), NjError> {
        napi_call_result!(crate::sys::napi_reject_deferred(
            self.0, deferred, rejection
        ))
    }

    pub fn create_thread_safe_function(
        &self,
        name: &str,
        js_func: Option<napi_value>,
        call_js_cb: napi_threadsafe_function_call_js,
    ) -> Result<crate::ThreadSafeFunction, NjError> {
        use crate::sys::napi_create_threadsafe_function;

        let work_name = self.create_string_utf8(name)?;

        let mut tsfn = ptr::null_mut();

        trace!("trying to create threadsafe fn: {}", name);

        napi_call_result!(napi_create_threadsafe_function(
            self.inner(),
            js_func.unwrap_or(ptr::null_mut()),
            ptr::null_mut(),
            work_name,
            0,
            1,
            ptr::null_mut(),
            None,
            ptr::null_mut(),
            call_js_cb,
            &mut tsfn
        ))?;

        trace!("created threadsafe fn: {}", name);

        Ok(crate::ThreadSafeFunction::new(self.0, tsfn))
    }

    pub fn is_exception_pending(&self) -> bool {
        let mut pending = false;
        napi_call_assert!(crate::sys::napi_is_exception_pending(
            self.inner(),
            &mut pending
        ));
        pending
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn throw(&self, value: napi_value) {
        debug!("throwing a native value");

        // check if there is exception pending, if so log and not do anything
        if self.is_exception_pending() {
            error!(
                "there is exception pending when trying to throw \
                 a native value, ignoring for now",
            );
            return;
        }

        unsafe { crate::sys::napi_throw(self.inner(), value) };
    }

    pub fn throw_type_error(&self, message: &str) {
        debug!(message, "type error");
        // check if there is exception pending, if so log and not do anything
        if self.is_exception_pending() {
            error!(
                "there is exception pending when trying to throw {}, ignoring for now",
                message
            );
            return;
        }

        let c_error_msg = CString::new(message).expect("message should not contain null");
        unsafe {
            crate::sys::napi_throw_type_error(self.inner(), ptr::null_mut(), c_error_msg.as_ptr())
        };
    }

    pub fn create_error(&self, message: &str) -> Result<napi_value, NjError> {
        let mut result = ptr::null_mut();

        let err_message = self.create_string_utf8(message)?;

        napi_call_result!(crate::sys::napi_create_error(
            self.0,
            ptr::null_mut(),
            err_message,
            &mut result
        ))?;

        Ok(result)
    }

    /// get value type
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn value_type(&self, napi_value: napi_value) -> Result<napi_valuetype, NjError> {
        use crate::sys::napi_typeof;

        let mut valuetype: napi_valuetype = 0;

        napi_call_result!(napi_typeof(self.inner(), napi_value, &mut valuetype))?;

        Ok(valuetype)
    }

    /// get string representation of value type
    pub fn value_type_string(&self, napi_value: napi_value) -> Result<&'static str, NjError> {
        Ok(napi_value_type_to_string(self.value_type(napi_value)?))
    }

    /// assert that napi value is certain type, otherwise raise exception
    pub fn assert_type(
        &self,
        napi_value: napi_value,
        should_be_type: napi_valuetype,
    ) -> Result<(), NjError> {
        let valuetype = self.value_type(napi_value)?;

        if valuetype != should_be_type {
            debug!(
                "value type is: {}-{} but should be: {}-{}",
                napi_value_type_to_string(valuetype),
                valuetype,
                napi_value_type_to_string(should_be_type),
                should_be_type
            );
            Err(NjError::InvalidType(
                napi_value_type_to_string(should_be_type).to_owned(),
                napi_value_type_to_string(valuetype).to_owned(),
            ))
        } else {
            Ok(())
        }
    }

    /// convert napi value to rust value
    pub fn convert_to_rust<'a, T>(&'a self, napi_value: napi_value) -> Result<T, NjError>
    where
        T: JSValue<'a>,
    {
        T::convert_to_rust(self, napi_value)
    }

    pub fn get_undefined(&self) -> Result<napi_value, NjError> {
        let mut result = ptr::null_mut();

        napi_call_result!(crate::sys::napi_get_undefined(self.0, &mut result))?;

        Ok(result)
    }
    pub fn get_null(&self) -> Result<napi_value, NjError> {
        let mut result = ptr::null_mut();
        napi_call_result!(crate::sys::napi_get_null(self.0, &mut result))?;
        Ok(result)
    }

    /// get buffer info
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn get_buffer_info(&self, napi_value: napi_value) -> Result<&[u8], NjError> {
        use std::slice;
        use crate::sys::napi_get_buffer_info;

        let mut len = 0_usize;
        let mut data = ptr::null_mut();

        //  napi_status napi_get_buffer_info(napi_env env,
        //      napi_value value,
        //      void** data,
        //      size_t* length)

        napi_call_result!(napi_get_buffer_info(
            self.inner(),
            napi_value,
            &mut data,
            &mut len
        ))?;

        let array: &[u8] = unsafe { slice::from_raw_parts(data as *const u8, len) };

        Ok(array)
    }

    /// Detach ArrayBuffer
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn detach_arraybuffer(&self, napi_value: napi_value) -> Result<(), NjError> {
        napi_call_result!(crate::sys::napi_detach_arraybuffer(
            self.inner(),
            napi_value
        ))?;
        Ok(())
    }

    /// Is this ArrayBuffer Detached?
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn is_detached_arraybuffer(&self, napi_value: napi_value) -> Result<bool, NjError> {
        let mut is_detached = false;
        napi_call_result!(crate::sys::napi_is_detached_arraybuffer(
            self.inner(),
            napi_value,
            &mut is_detached,
        ))?;
        Ok(is_detached)
    }

    #[allow(unused_unsafe)]
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn add_env_clean_up_hook(
        &self,
        init_func: Option<unsafe extern "C" fn(arg: *mut ::std::os::raw::c_void)>,
        arg: *mut ::std::os::raw::c_void,
    ) -> Result<(), NjError> {
        use crate::sys::napi_add_env_cleanup_hook;

        napi_call_result!(napi_add_env_cleanup_hook(self.inner(), init_func, arg))?;

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct JsCallback {
    env: JsEnv,
    this: napi_value,
    args: VecDeque<napi_value>,
}

unsafe impl Send for JsCallback {}
unsafe impl Sync for JsCallback {}

impl JsCallback {
    pub fn new(env: JsEnv, this: napi_value, args: Vec<napi_value>) -> Self {
        Self {
            env,
            this,
            args: args.into(),
        }
    }

    pub fn env(&self) -> &JsEnv {
        &self.env
    }

    pub fn args(&self, index: usize) -> napi_value {
        self.args[index]
    }

    pub fn this(&self) -> napi_value {
        self.this
    }

    pub fn this_owned(self) -> napi_value {
        self.this
    }

    pub fn remove_napi(&mut self) -> Option<napi_value> {
        self.args.remove(0)
    }

    /// consume next napi value and remove them from arg list
    pub fn get_value<'a, T>(&'a mut self) -> Result<T, NjError>
    where
        T: ExtractFromJs<'a>,
    {
        trace!(args = self.args.len(), "arg len");

        T::extract(self)
    }

    /// convert value to rust
    pub fn get_value_at<'a, T>(&'a self, index: usize) -> Result<T, NjError>
    where
        T: ExtractArgFromJs<'a>,
    {
        trace!(index, "trying extract value at");
        T::convert_arg_at(self, index)
    }

    /// create thread safe function
    pub fn create_thread_safe_function(
        &mut self,
        name: &str,
        call_js_cb: napi_threadsafe_function_call_js,
    ) -> Result<crate::ThreadSafeFunction, NjError> {
        if let Some(n_value) = self.remove_napi() {
            self.env
                .create_thread_safe_function(name, Some(n_value), call_js_cb)
        } else {
            Err(NjError::Other("expected js callback".to_owned()))
        }
    }

    /// create thread safe function at
    pub fn create_thread_safe_function_at(
        &self,
        name: &str,
        index: usize,
        call_js_cb: napi_threadsafe_function_call_js,
    ) -> Result<crate::ThreadSafeFunction, NjError> {
        if index < self.args.len() {
            self.env
                .create_thread_safe_function(name, Some(self.args[index]), call_js_cb)
        } else {
            Err(NjError::Other(format!("expected js callback at: {index}")))
        }
    }

    #[instrument]
    pub fn unwrap_mut<T>(&self) -> Result<&'static mut T, NjError> {
        Ok(self
            .env
            .unwrap_mut::<JSObjectWrapper<T>>(self.this())?
            .mut_inner())
    }

    pub fn unwrap<T>(&self) -> Result<&'static T, NjError> {
        Ok(self.env.unwrap::<JSObjectWrapper<T>>(self.this())?.inner())
    }
}

/// #[deprecated(since = "4.1.0","No longer used"]
pub trait ExtractFromJs<'a>: Sized {
    fn label() -> &'static str {
        std::any::type_name::<Self>()
    }

    /// extract from js callback
    fn extract(js_cb: &'a mut JsCallback) -> Result<Self, NjError>;
}

impl<'a, T: ?Sized> ExtractFromJs<'a> for T
where
    T: JSValue<'a>,
{
    fn label() -> &'static str {
        T::label()
    }

    #[instrument]
    fn extract(js_cb: &'a mut JsCallback) -> Result<Self, NjError> {
        trace!("extract from ExtractFromJs");
        if let Some(n_value) = js_cb.remove_napi() {
            T::convert_to_rust(js_cb.env(), n_value)
        } else {
            Err(NjError::Other(format!(
                "expected argument of type: {}",
                Self::label()
            )))
        }
    }
}

/// for optional argument
impl<'a, T: Sized> ExtractFromJs<'a> for Option<T>
where
    T: JSValue<'a>,
{
    fn label() -> &'static str {
        T::label()
    }

    fn extract(js_cb: &'a mut JsCallback) -> Result<Self, NjError> {
        if let Some(n_value) = js_cb.remove_napi() {
            Ok(Some(T::convert_to_rust(js_cb.env(), n_value)?))
        } else {
            Ok(None)
        }
    }
}

impl ExtractFromJs<'_> for JsEnv {
    fn extract(js_cb: &mut JsCallback) -> Result<Self, NjError> {
        Ok(*js_cb.env())
    }
}

pub trait ExtractArgFromJs<'a>: Sized {
    fn label() -> &'static str {
        std::any::type_name::<Self>()
    }

    /// convert js callback argument at index
    fn convert_arg_at(js_cb: &'a JsCallback, index: usize) -> Result<Self, NjError>;
}

impl<'a, T: ?Sized> ExtractArgFromJs<'a> for T
where
    T: JSValue<'a>,
{
    fn label() -> &'static str {
        T::label()
    }

    #[instrument]
    fn convert_arg_at(js_cb: &'a JsCallback, index: usize) -> Result<Self, NjError> {
        trace!(
            ty = std::any::type_name::<T>(),
            "extract from ExtractArgFromJs"
        );
        if index < js_cb.args.len() {
            T::convert_to_rust(js_cb.env(), js_cb.args[index])
        } else {
            Err(NjError::Other(format!(
                "trying to get arg at: {} but only {} args passed",
                index,
                js_cb.args.len()
            )))
        }
    }
}

impl<'a, T: Sized> ExtractArgFromJs<'a> for Option<T>
where
    T: JSValue<'a>,
{
    fn label() -> &'static str {
        T::label()
    }

    fn convert_arg_at(js_cb: &'a JsCallback, index: usize) -> Result<Self, NjError> {
        if index < js_cb.args.len() {
            Ok(Some(T::convert_to_rust(js_cb.env(), js_cb.args[index])?))
        } else {
            Ok(None)
        }
    }
}

impl ExtractArgFromJs<'_> for JsEnv {
    fn convert_arg_at(js_cb: &JsCallback, _index: usize) -> Result<Self, NjError> {
        Ok(*js_cb.env())
    }
}

#[derive(Debug)]
pub struct JsExports {
    inner: napi_value,
    env: JsEnv,
}

impl JsExports {
    pub fn new(env: napi_env, exports: napi_value) -> Self {
        Self {
            inner: exports,
            env: JsEnv::new(env),
        }
    }

    pub fn env(&self) -> &JsEnv {
        &self.env
    }

    pub fn prop_builder(&self) -> PropertiesBuilder {
        PropertiesBuilder::new()
    }

    pub fn define_property(&self, properties: PropertiesBuilder) -> Result<(), NjError> {
        // it is important not to release properties until this call is executed
        // since it is source of name string
        let mut raw_properties = properties.as_raw_properties();

        napi_call_result!(crate::sys::napi_define_properties(
            self.env.inner(),
            self.inner,
            raw_properties.len(),
            raw_properties.as_mut_ptr()
        ))
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn set_name_property(&self, name: &str, js_class: napi_value) -> Result<(), NjError> {
        let c_name = CString::new(name).expect("should work");

        napi_call_result!(crate::sys::napi_set_named_property(
            self.env.inner(),
            self.inner,
            c_name.as_ptr(),
            js_class
        ))
    }
}

/// Js callback function
pub struct JsCallbackFunction {
    ctx: napi_value,
    js_func: napi_value,
    env: JsEnv,
}

unsafe impl Send for JsCallbackFunction {}
unsafe impl Sync for JsCallbackFunction {}

impl JSValue<'_> for JsCallbackFunction {
    fn label() -> &'static str {
        "callback"
    }

    fn convert_to_rust(env: &JsEnv, js_value: napi_value) -> Result<Self, NjError> {
        env.assert_type(js_value, crate::sys::napi_valuetype_napi_function)?;

        let ctx = env.get_global()?;
        Ok(Self {
            ctx,
            js_func: js_value,
            env: *env,
        })
    }
}

impl JsCallbackFunction {
    /// invoke
    pub fn call<T>(&self, rust_argv: Vec<T>) -> Result<napi_value, NjError>
    where
        T: TryIntoJs,
    {
        trace!("invoking normal js callback");

        let env = &self.env;
        let mut argv: Vec<napi_value> = vec![];
        for rust_arg in rust_argv {
            match rust_arg.try_to_js(env) {
                Ok(js_value) => argv.push(js_value),
                Err(err) => return Err(err),
            }
        }

        self.env.call_function(self.ctx, self.js_func, argv)
    }
}

/// represent arbitrary js object
pub struct JsObject {
    env: JsEnv,
    napi_value: napi_value,
}

unsafe impl Send for JsObject {}

impl JsObject {
    /// create js object from js object
    pub fn new(env: JsEnv, napi_value: napi_value) -> Self {
        Self { env, napi_value }
    }

    /// create new js object from env
    pub fn create(env: &JsEnv) -> Result<Self, NjError> {
        Ok(Self::new(*env, env.create_object()?))
    }

    pub fn env(&self) -> &JsEnv {
        &self.env
    }

    pub fn napi_value(&self) -> napi_value {
        self.napi_value
    }

    /// get property
    pub fn get_property(&self, key: &str) -> Result<Option<Self>, NjError> {
        let property_key = self.env.create_string_utf8(key)?;

        let mut exist: bool = false;
        napi_call_result!(napi_has_property(
            self.env.inner(),
            self.napi_value,
            property_key,
            &mut exist,
        ))?;

        if !exist {
            return Ok(None);
        }

        let mut property_value = ptr::null_mut();

        napi_call_result!(napi_get_property(
            self.env.inner(),
            self.napi_value,
            property_key,
            &mut property_value,
        ))?;

        Ok(Some(Self {
            env: self.env,
            napi_value: property_value,
        }))
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn set_property(&mut self, key: &str, property_value: napi_value) -> Result<(), NjError> {
        use crate::sys::napi_set_property;

        let property_key = self.env.create_string_utf8(key)?;

        napi_call_result!(napi_set_property(
            self.env.inner(),
            self.napi_value,
            property_key,
            property_value,
        ))?;

        Ok(())
    }

    /// convert to equivalent rust object
    pub fn as_value<'a, T>(&'a self) -> Result<T, NjError>
    where
        T: JSValue<'a>,
    {
        self.env.convert_to_rust(self.napi_value)
    }
}

impl JSValue<'_> for JsObject {
    fn convert_to_rust(env: &JsEnv, js_value: napi_value) -> Result<Self, NjError> {
        env.assert_type(js_value, crate::sys::napi_valuetype_napi_object)?;
        Ok(Self::new(*env, js_value))
    }
}

impl TryIntoJs for JsObject {
    fn try_to_js(self, _js_env: &JsEnv) -> Result<napi_value, NjError> {
        Ok(self.napi_value)
    }
}
