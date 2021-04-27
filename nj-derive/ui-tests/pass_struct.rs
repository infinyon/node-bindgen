use node_bindgen::derive::node_bindgen;

#[node_bindgen]
struct Something {
    pub field: usize
}

#[node_bindgen]
pub(crate) struct WithVisibility {
    pub field: usize
}

#[node_bindgen]
struct Lifetime<'a> {
    pub field: &'a usize
}

#[node_bindgen]
struct BoundGeneric<T>
    where T: Sync + std::fmt::Debug + node_bindgen::core::TryIntoJs
{
    pub field: T
}

#[node_bindgen]
struct BoundAndLifetimes<'a, T: Sync + std::fmt::Debug + node_bindgen::core::TryIntoJs + Clone> {
    pub field: &'a T
}

#[node_bindgen]
struct Simple {
    pub a_string: String,
    pub a_number: i64,
    pub a_float : f64
}

#[node_bindgen]
struct Unnamed(String, f64);

#[node_bindgen]
struct UnitStruct;

fn main() {
}