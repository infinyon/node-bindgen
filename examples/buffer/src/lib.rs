use serde::Serialize;

use node_bindgen::derive::node_bindgen;
use node_bindgen::core::buffer::ArrayBuffer;
use node_bindgen::core::NjError;


#[derive(Serialize)]
struct MyStruct {
    a: String,
    b: i32
}

/// byte array buffer from json bytes
#[node_bindgen]
fn test(b: i32) -> Result<ArrayBuffer,NjError> {

    let my_struct = MyStruct {
        a: "b".to_string(),
        b
    };

    let json_string = serde_json::to_vec(&my_struct)
        .map_err(|err| NjError::Other(format!("serialization error: {}",err.to_string())))?;

    Ok(ArrayBuffer::new(json_string))
}


use node_bindgen::core::val::JsEnv;
use node_bindgen::core::TryIntoJs;
use node_bindgen::core::val::JsObject;
use node_bindgen::sys::napi_value;

struct Record {
    buffer: ArrayBuffer,
    comment: String
}


impl TryIntoJs for Record {

    /// serialize into json object
    fn try_to_js(self, js_env: &JsEnv) -> Result<napi_value,NjError> {

        // create JSON
        let mut json = JsObject::create(js_env)?;

        json.set_property("buffer",self.buffer.try_to_js(js_env)?)?;
        json.set_property("comment",js_env.create_string_utf8(&self.comment)?)?;
        
        json.try_to_js(js_env)
    }
}



/// create byte array and wrap in side another json obj
#[node_bindgen]
fn test2(b: i32) -> Result<Record,NjError> {

    let my_struct = MyStruct {
        a: "b".to_string(),
        b
    };

    let json_string = serde_json::to_vec(&my_struct)
        .map_err(|err| NjError::Other(format!("serialization error: {}",err.to_string())))?;

    Ok(Record {
        buffer: ArrayBuffer::new(json_string),
        comment: "array buffer is cool!".to_owned()
    })
}





#[node_bindgen]
fn test3(data: &[u8]) -> Result<String,NjError> {

    let message = String::from_utf8(data.to_vec())?;
    Ok(format!("reply {}",message))
}



#[node_bindgen]
fn test4(data: &[u8], foo: &[u8]) -> Result<String,NjError> {

    let message = String::from_utf8(data.to_vec())?;
    let f = String::from_utf8(foo.to_vec())?;

    Ok(format!("reply {} {}",message, f))
}


/*
extern "C" fn napi_test4(
    env: node_bindgen::sys::napi_env,
    cb_info: node_bindgen::sys::napi_callback_info,
) -> node_bindgen::sys::napi_value {
    use node_bindgen::core::TryIntoJs;
    use node_bindgen::core::IntoJs;
    use node_bindgen::core::val::JsCallbackFunction;
    fn test4(data: &[u8], foo: &[u8]) -> Result<String, NjError> {
        let message = String::from_utf8(data.to_vec())?;
        let f = String::from_utf8(foo.to_vec())?;
        Ok(format!("reply {} {}",message, f))
    }
    let js_env = node_bindgen::core::val::JsEnv::new(env);
    let result: Result<node_bindgen::sys::napi_value, node_bindgen::core::NjError> = (move || {
        let js_cb = js_env.get_cb_info(cb_info, 2)?;
        let rust_value_0 = js_cb.get_value_at::<&[u8]>(0)?;
        let rust_value_1 = js_cb.get_value_at::<&[u8]>(1)?;
        test4(rust_value_0, rust_value_1).try_to_js(&js_env)
    })();
    result.to_js(&js_env)
}
*/