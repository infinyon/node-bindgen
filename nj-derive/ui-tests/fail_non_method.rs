use node_bindgen::derive::node_bindgen;

 
#[node_bindgen(constructor)]
fn example2(count: i32) -> i32 {        
    count
}

#[node_bindgen(getter)]
fn example3(count: i32) -> i32 {        
    count
}

#[node_bindgen(setter)]
fn example4(count: i32) -> i32 {        
    count
}

fn main() {
    
}