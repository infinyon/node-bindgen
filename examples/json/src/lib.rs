use node_bindgen::derive::node_bindgen;
use node_bindgen::sys::napi_value;
use node_bindgen::core::NjError;
use node_bindgen::core::val::JsEnv;
use node_bindgen::core::TryIntoJs;
use node_bindgen::core::val::JsObject;

struct MyJson {
    val: f64,
}

impl TryIntoJs for MyJson {
    /// serialize into json object
    fn try_to_js(self, js_env: &JsEnv) -> Result<napi_value, NjError> {
        // create JSON
        let mut json = JsObject::new(js_env.clone(), js_env.create_object()?);

        let js_val = js_env.create_double(self.val)?;
        json.set_property("val", js_val)?;

        json.try_to_js(js_env)
    }
}

/// return json object
#[node_bindgen]
fn json() -> MyJson {
    MyJson { val: 10.0 }
}
