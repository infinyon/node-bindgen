use node_bindgen::derive::node_bindgen;
use node_bindgen::core::val::JsEnv;
use node_bindgen::core::val::JsObject;
use node_bindgen::core::JSValue;
use node_bindgen::sys::napi_value;
use node_bindgen::core::NjError;

#[derive(Default)]
struct Json {
    val: i32,
    name: Option<String>
}


/// accept integer 
/// or json
enum MyParam {
    Val(i32),
    Json(Json)
}

impl JSValue<'_> for MyParam {

    fn convert_to_rust(env: &JsEnv,n_value: napi_value) -> Result<Self,NjError> {

        // check if it is integer
        if let Ok(int_value) = env.convert_to_rust::<i32>(n_value) {
            Ok(Self::Val(int_value))
        }  else if  let Ok(js_obj) = env.convert_to_rust::<JsObject>(n_value) {
            let mut json = Json::default();
            if let Some(val_property) =  js_obj.get_property("val")? {
                json.val = val_property.as_value::<i32>()?;
                if let Some(name_property) = js_obj.get_property("name")? {
                    json.name = Some(name_property.as_value::<String>()?);
                }
                Ok(Self::Json(json))
            } else {
                Err(NjError::Other("val is not found".to_owned()))
            }
        } else {
            Err(NjError::Other("not valid format".to_owned()))
        }


    }
}

/// accept argument either int or json 
#[node_bindgen]
fn add(arg_opt: Option<MyParam>) -> i32 {

    if let Some(arg) = arg_opt {
        match arg {
            MyParam::Val(val) => val * 10,
            MyParam::Json(json) => json.val * 10
        }
    } else {
        0
    }
    
}