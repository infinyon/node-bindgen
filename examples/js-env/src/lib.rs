use node_bindgen::derive::node_bindgen;
use node_bindgen::sys::napi_value;
use node_bindgen::core::NjError;
use node_bindgen::core::val::JsEnv;

/// example where we receive napi callback manually
/// in order to do that, we use TryIntoJs trait
#[node_bindgen]
fn double(arg: f64) -> Result<EnvInterceptor, NjError> {
    println!("arg: {arg}");
    Ok(EnvInterceptor(arg))
}

struct EnvInterceptor(f64);

use node_bindgen::core::TryIntoJs;

impl TryIntoJs for EnvInterceptor {
    fn try_to_js(self, js_env: &JsEnv) -> Result<napi_value, NjError> {
        println!("intercepting env");
        js_env.create_double(self.0 * 2.0)
    }
}

