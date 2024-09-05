mod basic;
mod error;
mod thread_fn;
mod property;
mod class;
mod worker;
mod convert;
mod module;
pub mod buffer;
pub mod bigint;
pub mod stream;
pub mod safebuffer;

pub use thread_fn::ThreadSafeFunction;
pub use error::*;
pub use property::Property;
pub use property::PropertiesBuilder;
pub use class::JSClass;
pub use worker::create_promise;
pub use worker::JsPromiseFuture;
pub use worker::NjFutureExt;
pub use convert::*;
pub use ctor::ctor;
pub use module::submit_property;
pub use module::submit_register_callback;

use class::JSObjectWrapper;

pub mod sys {
    pub use nj_sys::*;
}
pub mod init {
    pub use ctor::ctor as node_bindgen_init_once;
}

pub mod future {
    pub use fluvio_future::task::spawn;
}

pub mod val {
    pub use crate::basic::*;
}

pub mod log {
    pub use tracing::*;
}

/// call napi and assert
/// used only in this crate
#[macro_export]
macro_rules! napi_call_assert {
    ($napi_expr:expr) => {{
        #[allow(clippy::macro_metavars_in_unsafe)]
        let status = unsafe { $napi_expr };
        if status != $crate::sys::napi_status_napi_ok {
            let nj_status: $crate::NapiStatus = status.into();
            tracing::error!("error executing napi call {:#?}", nj_status);
        }
    }};
}

/// call napi and wrap into result
/// used only in this crate
#[macro_export]
macro_rules! napi_call_result {
    ($napi_expr:expr) => {{
        #[allow(clippy::macro_metavars_in_unsafe)]
        let status = unsafe { $napi_expr };
        if status == $crate::sys::napi_status_napi_ok {
            Ok(())
        } else {
            let nj_status: $crate::NapiStatus = status.into();
            tracing::error!("node-bindgen error {:#?}", nj_status);
            Err(NjError::NapiCall(nj_status))
        }
    }};
}

/// convert result into napi value if ok otherwise convert to error
#[macro_export]
macro_rules! result_to_napi {
    ($result:expr) => {
        match $result {
            Ok(val) => val,
            Err(err) => return err.into(),
        }
    };
}

#[macro_export]
macro_rules! callback_to_napi {
    ($result:expr,$js_env:expr) => {
        match $result {
            Ok(val) => val,
            Err(err) => {
                return;
            }
        }
    };
}

/// assert the napi call
#[macro_export]
macro_rules! assert_napi {
    ($result:expr) => {
        match $result {
            Ok(val) => val,
            Err(err) => {
                panic!("napi call failed: {}", err)
            }
        }
    };
}

#[macro_export]
macro_rules! c_str {
    ($string:literal) => {{
        const _C_STRING: &'static str = concat!($string, "\0");
        _C_STRING
    }};
}

#[macro_export]
macro_rules! method {
    ($name:literal,$rs_method:expr) => {{
        nj::core::Property::new($name).method($rs_method)
    }};
}
