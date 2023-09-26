use node_bindgen::derive::node_bindgen;
use node_bindgen::sys::napi_value;
use node_bindgen::core::NjError;
use node_bindgen::core::val::JsEnv;
use node_bindgen::core::TryIntoJs;
use node_bindgen::core::val::JsObject;

use serde_json::value::Value;
use serde_json::map::Map;

// The recommended way of transforming to json
#[node_bindgen]
struct StandardJson {
    some_name: String,
    a_number: i64,
}

#[node_bindgen]
struct Outer {
    val: Inner,
}

#[node_bindgen]
struct Inner(String);

#[node_bindgen]
struct UnitStruct;

#[node_bindgen]
enum ErrorType {
    WithMessage(String, usize),
    WithFields { val: usize },
    UnitError,
}

#[node_bindgen]
struct WithSerdeJson {
    val: Value,
}

struct CustomJson {
    val: f64,
}

impl TryIntoJs for CustomJson {
    /// serialize into json object, with custom field names
    fn try_to_js(self, js_env: &JsEnv) -> Result<napi_value, NjError> {
        // create JSON
        let mut json = JsObject::new(*js_env, js_env.create_object()?);

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
        a_number: 1337,
    }
}

#[node_bindgen]
fn multilevel_json() -> Outer {
    Outer {
        val: Inner("hello".to_owned()),
    }
}

#[node_bindgen]
fn unit_struct() -> UnitStruct {
    UnitStruct
}

#[node_bindgen]
fn with_message() -> ErrorType {
    ErrorType::WithMessage("test".to_owned(), 321)
}

#[node_bindgen]
fn with_fields() -> ErrorType {
    ErrorType::WithFields { val: 123 }
}

#[node_bindgen]
fn with_unit() -> ErrorType {
    ErrorType::UnitError
}

#[node_bindgen]
fn failed_result_with_fields() -> Result<(), ErrorType> {
    Err(ErrorType::WithFields { val: 987 })
}

#[node_bindgen]
async fn async_result_failed_unit() -> Result<(), ErrorType> {
    Err(ErrorType::UnitError)
}

#[node_bindgen]
fn with_serde_json() -> WithSerdeJson {
    let mut map = Map::new();
    map.insert("first".to_owned(), Value::Bool(true));
    map.insert("second".to_owned(), Value::String("hello".to_owned()));

    WithSerdeJson {
        val: Value::Object(map),
    }
}
