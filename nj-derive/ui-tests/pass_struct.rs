use node_bindgen::derive::node_bindgen;

#[node_bindgen]
struct Something {
    pub field: usize
}

#[node_bindgen]
pub(crate) struct SomethingElse {
    pub field: usize
}

fn main() {
}