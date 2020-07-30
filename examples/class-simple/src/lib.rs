use node_bindgen::derive::node_bindgen;


struct MyObject {
    val: f64,
    val2: i64
}

#[node_bindgen]
impl MyObject {
    #[node_bindgen(constructor)]
    fn new(val: f64,val2: i64) -> Self {
        Self { val, val2 }
    }

    /*
    /// simple method which return f64
    /// rust values are automatically converted into equivalent JS value
    /// method name are generated from rust method name
    /// Js:  let y = obj.plusOne();
    #[node_bindgen]
    fn plus_one(&self) -> f64 {
        self.val + 1.0
    }

    /// JS getter
    /// Js:  let y = obj.value;
    #[node_bindgen(getter)]
    fn value(&self) -> f64 {
        self.val
    }

    /// JS getter
    /// Js:  let y = obj.value2;
    #[node_bindgen(getter)]
    fn value2(&self) -> i64 {
        self.val2
    }

    /// JS Setter
    /// Js:  obj.value3 = 10;
    #[node_bindgen(setter)]
    fn value3(&mut self, val: f64) {
        self.val = val;
    }

    /// method with custom name instead of generated name
    /// Js:  obj.updateValue(10);
    #[node_bindgen(name = "updateValue")]
    fn set_value(&mut self, val: f64) {
        self.val = val;
    }

    #[node_bindgen(setter, name = "value4")]
    fn set_value4(&mut self, val: f64) {
        self.val = val;
    }

    #[node_bindgen]
    fn change_value(&mut self, val: f64) {
        self.val = val;
    }

    #[node_bindgen(getter)]
    fn is_positive(&self) -> bool {
        self.val > 0.0
    }

    #[node_bindgen(setter)]
    fn clear(&mut self, val: bool) {
        if val {
            self.val = 0.0;
        }
    }
    */
}

