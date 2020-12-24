use std::time::Duration;

use fluvio_future::timer::sleep;
use node_bindgen::derive::node_bindgen;


#[node_bindgen]
async fn basic<F: Fn(f64,f64)>( seconds: i32, cb: F) {
 
    sleep(Duration::from_secs(1)).await;
    cb(seconds as f64,(seconds*2) as f64);

}


#[node_bindgen]
async fn hello<F: Fn(f64,String)>( seconds: i32, cb: F) {

  //  println!("sleeping");
    sleep(Duration::from_secs(seconds as u64)).await;
//    println!("woke from time");

    cb(10.0,"hello world".to_string());

}




/*
extern "C" fn napi_hello(
    env: node_bindgen::sys::napi_env,
    cb_info: node_bindgen::sys::napi_callback_info,
) -> node_bindgen::sys::napi_value {
    use node_bindgen::core::TryIntoJs;
    use node_bindgen::core::IntoJs;
    use node_bindgen::core::val::JsCallbackFunction;
    async fn hello<F: Fn(f64, String)>(seconds: i32, cb: F) {
        sleep(Duration::from_secs(seconds as u64)).await;
        cb(10.0, "hello world".to_string());
    }
    struct Argcb {
        arg0: f64,
        arg1: String,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Argcb {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Argcb {
                    arg0: ref __self_0_0,
                    arg1: ref __self_0_1,
                } => {
                    let mut debug_trait_builder = f.debug_struct("Argcb");
                    let _ = debug_trait_builder.field("arg0", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("arg1", &&(*__self_0_1));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    extern "C" fn thread_safe_cb_complete(
        env: node_bindgen::sys::napi_env,
        js_cb: node_bindgen::sys::napi_value,
        _context: *mut ::std::os::raw::c_void,
        data: *mut ::std::os::raw::c_void,
    ) {
        if env != std::ptr::null_mut() {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    let _ = ::core::fmt::Arguments::new_v1(
                        &["async cb invoked"],
                        &match () {
                            () => [],
                        },
                    );
                    ::log::__private_api_log_lit(
                        "async cb invoked",
                        lvl,
                        &(
                            "nj_example_async_cb",
                            "nj_example_async_cb",
                            "async-cb/src/lib.rs",
                            16u32,
                        ),
                    );
                }
            };
            let js_env = node_bindgen::core::val::JsEnv::new(env);
            let result: Result<(), node_bindgen::core::NjError> = (move || {
                let global = js_env.get_global()?;
                let my_val: Box<Argcb> = unsafe { Box::from_raw(data as *mut Argcb) };
                {
                    let lvl = ::log::Level::Trace;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api_log(
                            ::core::fmt::Arguments::new_v1_formatted(
                                &["arg: "],
                                &match (&my_val,) {
                                    (arg0,) => [::core::fmt::ArgumentV1::new(
                                        arg0,
                                        ::core::fmt::Debug::fmt,
                                    )],
                                },
                                &[::core::fmt::rt::v1::Argument {
                                    position: 0usize,
                                    format: ::core::fmt::rt::v1::FormatSpec {
                                        fill: ' ',
                                        align: ::core::fmt::rt::v1::Alignment::Unknown,
                                        flags: 4u32,
                                        precision: ::core::fmt::rt::v1::Count::Implied,
                                        width: ::core::fmt::rt::v1::Count::Implied,
                                    },
                                }],
                            ),
                            lvl,
                            &(
                                "nj_example_async_cb",
                                "nj_example_async_cb",
                                "async-cb/src/lib.rs",
                                16u32,
                            ),
                        );
                    }
                };
                let js_arg0 = my_val.arg0.try_to_js(&js_env)?;
                let js_arg1 = my_val.arg1.try_to_js(&js_env)?;
                {
                    let lvl = ::log::Level::Debug;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        let _ = ::core::fmt::Arguments::new_v1(
                            &["async cb, invoking js cb"],
                            &match () {
                                () => [],
                            },
                        );
                        ::log::__private_api_log_lit(
                            "async cb, invoking js cb",
                            lvl,
                            &(
                                "nj_example_async_cb",
                                "nj_example_async_cb",
                                "async-cb/src/lib.rs",
                                16u32,
                            ),
                        );
                    }
                };
                js_env.call_function(global, js_cb, <[_]>::into_vec(box [js_arg0, js_arg1]))?;
                {
                    let lvl = ::log::Level::Trace;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        let _ = ::core::fmt::Arguments::new_v1(
                            &["async cb, done"],
                            &match () {
                                () => [],
                            },
                        );
                        ::log::__private_api_log_lit(
                            "async cb, done",
                            lvl,
                            &(
                                "nj_example_async_cb",
                                "nj_example_async_cb",
                                "async-cb/src/lib.rs",
                                16u32,
                            ),
                        );
                    }
                };
                Ok(())
            })();
            match result {
                Ok(val) => val,
                Err(err) => ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                    &["napi call failed: "],
                    &match (&err,) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                )),
            }
        }
    }
    let js_env = node_bindgen::core::val::JsEnv::new(env);
    let result: Result<node_bindgen::sys::napi_value, node_bindgen::core::NjError> = (move || {
        let mut js_cb = js_env.get_cb_info(cb_info, 2)?;
        let rust_value_0 = js_cb.get_value_at::<i32>(0)?;
        let rust_value_1 =
            js_cb.create_thread_safe_function("hello_sf", Some(thread_safe_cb_complete))?;
        (node_bindgen::core::JsPromiseFuture::new(
            hello(rust_value_0, move |cb_arg0: f64, cb_arg1: String| {
                let arg = Argcb {
                    arg0: cb_arg0,
                    arg1: cb_arg1,
                };
                {
                    let lvl = ::log::Level::Trace;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        let _ = ::core::fmt::Arguments::new_v1(
                            &["converting rust to raw ptr"],
                            &match () {
                                () => [],
                            },
                        );
                        ::log::__private_api_log_lit(
                            "converting rust to raw ptr",
                            lvl,
                            &(
                                "nj_example_async_cb",
                                "nj_example_async_cb",
                                "async-cb/src/lib.rs",
                                16u32,
                            ),
                        );
                    }
                };
                let my_box = Box::new(arg);
                let ptr = Box::into_raw(my_box);
                rust_value_1
                    .call(Some(ptr as *mut core::ffi::c_void))
                    .expect("callback should work");
            }),
            "hello_ft",
        ))
        .try_to_js(&js_env)
    })();
    result.to_js(&js_env)
}
*/