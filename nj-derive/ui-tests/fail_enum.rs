use node_bindgen::derive::node_bindgen;

#[node_bindgen]
enum TestEnum {
    Something,
    Else
}