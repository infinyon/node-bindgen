use node_bindgen::derive::node_bindgen;
use node_bindgen::sys::napi_value;
use node_bindgen::core::NjError;
use node_bindgen::core::val::JsEnv;

/// example where we receive napi callback manually
/// with napi callback, have full control over JS object lifecycle
/// JsEnv argument does not manipulate JsCb arguments
#[node_bindgen]
fn multiply(env: JsEnv, arg: f64) -> Result<napi_value, NjError> {
    println!("arg: {}", arg);
    env.create_double(arg * 2.0)
}
