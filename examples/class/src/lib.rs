
use std::time::Duration;
use std::io::Error as IoError;

use flv_future_aio::timer::sleep;

use node_bindgen::sys::napi_value;
use node_bindgen::core::val::JsCallback;
use node_bindgen::core::JSClass;
use node_bindgen::core::NjError;
use node_bindgen::core::val::JsObject;
use node_bindgen::core::val::JsEnv;
use node_bindgen::core::TryIntoJs;
use node_bindgen::derive::node_bindgen;

#[node_bindgen]
async fn create(val: f64) -> Result<MyObjectWrapper,IoError> {
    Ok(MyObjectWrapper{ val })
}

struct MyObjectWrapper {
    val: f64
}

impl TryIntoJs for MyObjectWrapper {

    fn try_to_js(self, js_env: &JsEnv) -> Result<napi_value,NjError> {
        let instance = TestObject::new_instance(js_env,vec![])?;
        let test_object = TestObject::unwrap_mut(js_env,instance)?;
        test_object.set_value(self.val);
        Ok(instance)   
    }
}

struct TestObject {
    val: Option<f64>
}

#[node_bindgen]
impl TestObject {

    #[node_bindgen(constructor)]
    fn new() -> Self {
        Self { val: None }
    }

    #[node_bindgen(setter,name="value")]
    fn set_value(&mut self,val: f64) {
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

/*
impl JSValue for TestObject {

    const JS_TYPE: u32 = node_bindgen::sys::napi_valuetype_napi_object;

    fn convert_to_rust(env: &JsEnv,js_value: napi_value) -> Result<Self,NjError> {

        env.unwrap::<Self>(js_value)
    }

}
*/

struct MyObject {
    val: f64,
}


#[node_bindgen]
impl MyObject {

    #[node_bindgen(constructor)]
    fn new(val: f64) -> Self {
        Self { val }
    }

    /// simple method which return f64
    /// rust values are automatically converted into equivalent JS value
    /// method name are generated from rust method name
    /// Js:  let y = obj.plusOne();
    #[node_bindgen]
    fn plus_one(&self) -> f64 {
        self.val + 1.0
    }

    /// JS getter
    /// Js:  let y = obj.value;
    #[node_bindgen(getter)]
    fn value(&self) -> f64 {
        self.val
    }

    /// JS Setter
    /// Js:  obj.value3 = 10;
    #[node_bindgen(setter)]
    fn value3(&mut self,val: f64) {
        self.val = val;
    }

    /// method with custom name instead of generated name
    /// Js:  obj.set_value(10);
    #[node_bindgen(name="value2")]
    fn set_value(&mut self,val: f64) {
        self.val = val;
    }

    
    #[node_bindgen(setter,name="value4")]
    fn set_value4(&mut self,val: f64) {
        self.val = val;
    }
    

    #[node_bindgen]
    fn change_value(&mut self,val: f64)  {
        self.val = val;
    }

      
    #[node_bindgen(getter)]
    fn is_positive(&self) -> bool {
        self.val > 0.0
    }

    #[node_bindgen(setter)]
    fn clear(&mut self,val: bool) {
        if val {
            self.val = 0.0;
        }
    }

    /// accept arbitrary js object, here we are looking integer property with value
    #[node_bindgen]
    fn plus_score(&mut self,config: JsObject) -> Result<f64,NjError> {

        let score_property = config.get_property("score")?;
        println!("score founded");
        let score = score_property.as_value::<f64>()?;
        Ok(self.val + score)
    }


    
    //// accept Rust object
    ///  Js: obj.plus_test(test)
    #[node_bindgen]
    fn plus_test(&mut self,config: &TestObject) -> Result<f64,NjError> {

        Ok(self.val + config.value2())
    }

    

    /// example where we receive callback cb explicitly. 
    /// in this case, we can manually create new instance.
    /// callback must be 2nd argument and be of type JsCallback.
    /// JS example: let obj2 = obj.multiply(-1);
    #[node_bindgen]
    fn multiply(&self, cb: &JsCallback, arg: f64) -> Result<napi_value, NjError> {
        
        let new_val = cb.env().create_double(arg * self.val)?;
        Self::new_instance(cb.env(), vec![new_val])
    } 



    
    /// promise which result in primitive type
    #[node_bindgen]
    async fn plus_two(&self, arg: f64) -> f64 {

        println!("sleeping");
        sleep(Duration::from_secs(1)).await;
        println!("woke and adding {}",arg);
        
        self.val + arg
    }
    

    /// promise where result is arbitrary struct.
    /// returning struct must implement TryIntoJs
    /// which can create new JS instance
    #[node_bindgen]
    async fn multiply2(&self,arg: f64) -> MyObjectConstructor {

        println!("sleeping");
        sleep(Duration::from_secs(1)).await;
        println!("woke and adding {}",arg);
        
        MyObjectConstructor::new(self.val * arg)
    }



    /// loop and emit event
    #[node_bindgen]
    async fn sleep<F: Fn(String)>(&self,cb: F)  {

        println!("sleeping");
        sleep(Duration::from_secs(1)).await;
        let msg = format!("hello world");
        cb(msg);        
        
    }

    #[node_bindgen]
    fn test(&self) -> f64 {
        0.0
    }


}
