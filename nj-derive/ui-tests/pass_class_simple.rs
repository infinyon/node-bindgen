use node_bindgen::derive::node_bindgen;

 
struct MyObject {
    val: f64,
}


#[node_bindgen]
impl MyObject {

    
    #[node_bindgen(constructor)]
    fn new(val: f64) -> Self {
        Self { val }
    }
    
    #[node_bindgen]
    fn twice(&self) -> f64 {
        self.val * 2.0
    }

    
    #[node_bindgen(getter)]
    fn value(&self) -> f64 {
        self.val
    }
    

    #[node_bindgen(name = "value2",getter)]
    fn set_value(&mut self, val: f64) {
        self.val = val;
    }

}

fn main() {
    
}