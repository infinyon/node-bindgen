
use node_bindgen::derive::node_bindgen;


#[node_bindgen]
fn hello(count: i32) -> String {        
    format!("hello world {}", count)
}


#[node_bindgen]
fn sum(first: i32, second: i32) -> i32 {        
    first + second
}

