use node_bindgen::derive::node_bindgen;
use node_bindgen::sys::napi_value;
use node_bindgen::core::NjError;
use node_bindgen::core::val::JsEnv;
use node_bindgen::core::TryIntoJs;
use node_bindgen::core::val::JsObject;

// The recommended way of transforming to json
#[node_bindgen]
struct StandardJson {
    some_name: String,
    a_number: i64
}

#[node_bindgen]
struct Outer {
    val: Inner
}

#[node_bindgen]
struct Inner(String);

struct CustomJson {
    val: f64
}

impl TryIntoJs for CustomJson {
    /// serialize into json object, with custom field names
    fn try_to_js(self, js_env: &JsEnv) -> Result<napi_value, NjError> {
        // create JSON
        let mut json = JsObject::new(js_env.clone(), js_env.create_object()?);

        let js_val = js_env.create_double(self.val)?;
        json.set_property("customFieldName", js_val)?;

        json.try_to_js(js_env)
    }
}

/// return json object
#[node_bindgen]
fn custom_json() -> CustomJson {
    CustomJson { val: 10.0 }
}

#[node_bindgen]
fn standard_json() -> StandardJson {
    StandardJson {
        some_name: "John".to_owned(),
        a_number: 1337
    }
}

#[node_bindgen]
fn multilevel_json() -> Outer {
    Outer {
        val: Inner("hello".to_owned())
    }
}