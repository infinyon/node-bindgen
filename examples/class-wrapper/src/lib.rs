use std::io::Error as IoError;

use node_bindgen::sys::napi_value;
use node_bindgen::core::JSClass;
use node_bindgen::core::NjError;
use node_bindgen::core::val::JsEnv;
use node_bindgen::core::TryIntoJs;
use node_bindgen::derive::node_bindgen;

/// simple wrapper
#[node_bindgen]
fn simple(val: f64) -> Result<TestObject, IoError> {
    Ok(TestObject { val: Some(val) })
}

impl TryIntoJs for TestObject {
    fn try_to_js(self, js_env: &JsEnv) -> Result<napi_value, NjError> {
        let instance = Self::new_instance(js_env, vec![])?;
        let test_object = Self::unwrap_mut(js_env, instance)?;
        test_object.set_value(self.val.unwrap());
        Ok(instance)
    }
}

/// indirect wrapper
#[node_bindgen]
async fn create(val: f64) -> Result<MyObjectWrapper, IoError> {
    Ok(MyObjectWrapper { val })
}

struct MyObjectWrapper {
    val: f64,
}

impl TryIntoJs for MyObjectWrapper {
    fn try_to_js(self, js_env: &JsEnv) -> Result<napi_value, NjError> {
        let instance = TestObject::new_instance(js_env, vec![])?;
        let test_object = TestObject::unwrap_mut(js_env, instance)?;
        test_object.set_value(self.val);
        Ok(instance)
    }
}

struct TestObject {
    val: Option<f64>,
}

#[node_bindgen]
impl TestObject {
    #[node_bindgen(constructor)]
    fn new() -> Self {
        Self { val: None }
    }

    #[node_bindgen(setter, name = "value")]
    fn set_value(&mut self, val: f64) {
        self.val.replace(val);
    }

    #[node_bindgen(getter)]
    fn value2(&self) -> f64 {
        self.val.unwrap_or(0.0)
    }

    #[node_bindgen]
    fn test(&self) -> f64 {
        0.0
    }
}
