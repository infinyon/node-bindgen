use node_bindgen::derive::node_bindgen;
 
/// name2 is not valid attribute
#[node_bindgen(name2="hello")]
fn example1(count: i32) -> String {        
    format!("hello world {}", count)
}

fn main() {
    
}