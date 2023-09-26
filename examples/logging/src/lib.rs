use tracing::{info, warn};

use node_bindgen::derive::node_bindgen;
use node_bindgen::init::node_bindgen_init_once;


#[node_bindgen_init_once]
fn init_logging() {
    // initialize logging framework
    // logging is initialized already
    info!("logging initialized");
}

#[node_bindgen()]
fn hello(count: i32) -> String {
    warn!("calling hello");
    format!("hello world {count}")
}
