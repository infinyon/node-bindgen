use node_bindgen::derive::node_bindgen;



struct MyObject {
    val: f64,
}


#[node_bindgen]
impl MyObject {

    #[node_bindgen(name2="hello")]
    fn new(val: f64) -> Self {
        Self { val }
    }

}


fn main() {
    
}