use std::ptr;

use tracing::error;
use tracing::trace;
use tracing::debug;
use futures_lite::Future;

use fluvio_future::task::spawn;

use crate::sys::napi_deferred;
use crate::sys::napi_value;
use crate::val::JsEnv;
use crate::NjError;
use crate::sys::napi_env;
use crate::TryIntoJs;
use crate::assert_napi;
use crate::ThreadSafeFunction;

pub struct JsPromiseFuture<F> {
    future: F,
    name: String,
}

impl<F> JsPromiseFuture<F>
where
    F: Future,
    F::Output: TryIntoJs,
{
    pub fn new<S>(future: F, name: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            future,
            name: name.into(),
        }
    }
}

impl<F> TryIntoJs for JsPromiseFuture<F>
where
    F: Future + 'static + Send,
    F::Output: TryIntoJs,
{
    fn try_to_js(self, js_env: &JsEnv) -> Result<napi_value, NjError> {
        create_promise(js_env, &self.name, self.future)
    }
}

struct JsDeferred(napi_deferred);
unsafe impl Send for JsDeferred {}

pub struct WorkerResult<O> {
    deferred: JsDeferred,
    result: O,
}

/// create promise and schedule work
/// when this is finished it will return result in the main thread
pub fn create_promise<F, O>(js_env: &JsEnv, name: &str, future: F) -> Result<napi_value, NjError>
where
    F: Future<Output = O> + 'static + Send,
    O: TryIntoJs,
{
    let (promise, deferred) = js_env.create_promise()?;
    let function_name = format!("async_worker_th_{name}");
    let ts_fn =
        js_env.create_thread_safe_function(&function_name, None, Some(promise_complete::<O>))?;
    let js_deferred = JsDeferred(deferred);

    spawn(async move {
        let result = future.await;
        finish_worker(ts_fn, result, js_deferred);
    });

    Ok(promise)
}

extern "C" fn promise_complete<O>(
    env: napi_env,
    _js_cb: napi_value,
    _context: *mut ::std::os::raw::c_void,
    data: *mut ::std::os::raw::c_void,
) where
    O: TryIntoJs,
{
    if !env.is_null() {
        trace!("promise complete");
        let js_env = JsEnv::new(env);

        let worker_result: Box<WorkerResult<O>> =
            unsafe { Box::from_raw(data as *mut WorkerResult<O>) };

        let result: Result<(), NjError> = match worker_result.result.try_to_js(&js_env) {
            Ok(val) => {
                trace!("trying to resolve to deferred");
                js_env.resolve_deferred(worker_result.deferred.0, val)
            }
            Err(js_err) => {
                trace!("trying to resolve to deferred");
                js_env.reject_deferred(worker_result.deferred.0, js_err.as_js(&js_env))
            }
        };
        assert_napi!(result)
    }
}

fn finish_worker<O>(ts_fn: ThreadSafeFunction, result: O, deferred: JsDeferred)
where
    O: TryIntoJs,
{
    let boxed_worker = Box::new(WorkerResult { deferred, result });
    let ptr = Box::into_raw(boxed_worker);
    if let Err(err) = ts_fn.call(Some(ptr as *mut core::ffi::c_void)) {
        error!("error finishing worker: {}", err);
    }
}

pub trait NjFutureExt: Future {
    fn try_to_js(self, js_env: &JsEnv) -> Result<napi_value, NjError>
    where
        Self: Sized + Send + 'static,
        Self::Output: TryIntoJs,
    {
        extern "C" fn promise_complete2<O>(
            env: napi_env,
            _js_cb: napi_value,
            _context: *mut ::std::os::raw::c_void,
            data: *mut ::std::os::raw::c_void,
        ) {
            if !env.is_null() {
                trace!("promise complete");
                let _ = JsEnv::new(env);

                let _: Box<O> = unsafe { Box::from_raw(data as *mut O) };
            }
        }

        let function_name = "stream_example_1".to_string();
        let _ = js_env.create_thread_safe_function(
            &function_name,
            None,
            Some(promise_complete2::<Self::Output>),
        )?;

        debug!("spawning task");
        spawn(async move {
            let _ = self.await;
            debug!("task completed");
        });

        Ok(ptr::null_mut())
    }
}

impl<T: ?Sized> NjFutureExt for T where T: Future {}
