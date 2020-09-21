
use std::time::Duration;


use fluvio_future::timer::sleep;

use node_bindgen::sys::napi_value;
use node_bindgen::core::NjError;
use node_bindgen::core::val::JsObject;
use node_bindgen::core::val::JsEnv;
use node_bindgen::core::TryIntoJs;
use node_bindgen::derive::node_bindgen;


struct MyJson {
    val: f64
}


impl TryIntoJs for MyJson {

    fn try_to_js(self, js_env: &JsEnv) -> Result<napi_value,NjError> {

        // create JSON
        let mut json = JsObject::new(js_env.clone(), js_env.create_object()?);

        let js_val = js_env.create_double(self.val)?;
        json.set_property("val",js_val)?;
        
        json.try_to_js(js_env)
    }
}



struct MyObject {
    val: f64,
}


#[node_bindgen]
impl MyObject {

    #[node_bindgen(constructor)]
    fn new(val: f64) -> Self {
        Self { val }
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


}
