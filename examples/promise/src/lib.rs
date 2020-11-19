use std::time::Duration;


use fluvio_future::timer::sleep;
use node_bindgen::derive::node_bindgen;
use node_bindgen::core::NjError;

#[node_bindgen]
async fn hello(arg: f64) -> f64 {
    println!("sleeping");
    sleep(Duration::from_secs(1)).await;
    println!("woke and adding 10.0");
    arg + 10.0
}

#[node_bindgen]
async fn hello2(arg: f64) -> Result<f64,NjError> {
    println!("sleeping");
    sleep(Duration::from_secs(1)).await;
    if arg < 0.0 {
        eprintln!("throwing error");
        Err(NjError::Other("arg is negative".to_owned()))
    } else {
        println!("woke and adding 10.0");
        Ok(arg + 10.0)
    }
   
}

/// just sleep
#[node_bindgen]
async fn just_sleep(seconds: i32) -> () {
    println!("sleeping");
    sleep(Duration::from_secs(seconds as u64)).await;
    println!("finished sleeping");
}

#[derive(Debug)]
struct NativeStore {
    val: String,
}

#[node_bindgen]
impl NativeStore {
    #[node_bindgen(constructor)]
    fn new() -> Self {
        Self { val: String::from("unknown") }
    }

    #[node_bindgen]
    async fn get(&self) -> String {
        sleep(std::time::Duration::from_micros(1)).await;
        self.val.clone()
    }

    #[node_bindgen]
    async fn put(&mut self, value: String) {
        sleep(std::time::Duration::from_millis(500)).await;
        self.val = value;
    }
}
