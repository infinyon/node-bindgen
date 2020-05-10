use node_bindgen::derive::node_bindgen;


#[node_bindgen]
fn example<F: Fn(i32)>(cb: F,second: i32) {        
    cb(second);
}

fn example2<F: Fn(String,i64)>(first: i32,cb: F)  {
    cb(format!("hello world: {}",first),first as i64);
}



fn main() {
}