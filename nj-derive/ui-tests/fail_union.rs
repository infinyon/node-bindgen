use node_bindgen::derive::node_bindgen;

#[node_bindgen]
union TestUnion {
    pub field1: u32,
    pub field2: f32
}