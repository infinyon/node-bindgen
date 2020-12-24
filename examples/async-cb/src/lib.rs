use std::time::Duration;

use fluvio_future::timer::sleep;
use node_bindgen::derive::node_bindgen;

#[node_bindgen]
async fn basic<F: Fn(f64, f64)>(seconds: i32, cb: F) {
    sleep(Duration::from_secs(1)).await;
    cb(seconds as f64, (seconds * 2) as f64);
}

#[node_bindgen]
async fn hello<F: Fn(f64, String)>(seconds: i32, cb: F) {
    //  println!("sleeping");
    sleep(Duration::from_secs(seconds as u64)).await;
    //    println!("woke from time");

    cb(10.0, "hello world".to_string());
}