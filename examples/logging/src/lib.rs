use node_bindgen::derive::node_bindgen;
use node_bindgen::init::node_bindgen_init_once;
use log::{info, warn};

#[node_bindgen_init_once]
fn init_logging() {
    // initialize logging framework
    env_logger::init();
    info!("logging initialized");
}

#[node_bindgen()]
fn hello(count: i32) -> String {
    warn!("calling hello");
    format!("hello world {}", count)
}
