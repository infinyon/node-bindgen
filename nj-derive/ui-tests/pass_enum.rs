use node_bindgen::derive::node_bindgen;
use node_bindgen::core::TryIntoJs;

#[node_bindgen]
enum TestEnum {
    Something(usize),
    Else {
        val: String
    },
    UnitVariant
}

#[node_bindgen]
enum Generic<T: TryIntoJs> {
    Container(T)
}

fn main() {
}