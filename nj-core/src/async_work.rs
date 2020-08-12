use std::ptr;
use log::debug;

use crate::sys::{napi_env, napi_value};
use crate::sys::napi_async_work;
use crate::sys::napi_async_execute_callback;
use crate::sys::napi_async_complete_callback;

use crate::napi_call_result;
use crate::NjError;
use crate::val::JsEnv;

pub struct AsyncWork {
    env: JsEnv,
    handle: napi_async_work
}

unsafe impl Sync for AsyncWork{}
unsafe impl Send for AsyncWork{}

impl AsyncWork {
    pub fn new<E>(env: E, handle: napi_async_work) -> Self 
        where E: Into<JsEnv>
    {
        Self {
            env: env.into(),
            handle
        }
    }

    pub fn inner(self) -> napi_async_work {
        self.handle
    }

    // pub fn env(&self) -> napi_value {
    //     self.env.inner()
    // }

    pub fn create<E>(
        env: E,
        async_resource: napi_value, 
        async_resource_name: napi_value, 
        execute: napi_async_execute_callback,
        complete: napi_async_complete_callback,
        data: *mut core::ffi::c_void
    ) -> Self 
    where 
        E: Into<JsEnv> {
        unimplemented!()
    }
}