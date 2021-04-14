use node_bindgen::derive::node_bindgen;
use node_bindgen::core::TryIntoJs;
use std::fmt::Debug;

#[node_bindgen]
struct Something {
    pub field: usize
}

#[node_bindgen]
pub(crate) struct WithVisibility {
    pub field: usize
}

#[node_bindgen]
struct Generic<T> {
    pub field: T
}

#[node_bindgen]
struct Lifetime<'a> {
    pub field: &'a usize
}

#[node_bindgen]
struct BoundGeneric<T>
    where T: Sync + Debug
{
    pub field: T
}

#[node_bindgen]
struct BoundAndLifetimes<'a, T: Sync + Debug> {
    pub field: &'a T
}

fn main() {
}