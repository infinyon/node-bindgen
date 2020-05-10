use node_bindgen::derive::node_bindgen;


/// async callback
#[node_bindgen]
async fn example5<F: Fn(f64,String)>( seconds: i32, cb: F) {
}


#[node_bindgen]
async fn example6(arg: f64) -> f64 {
    0.0
}



fn main() {
}