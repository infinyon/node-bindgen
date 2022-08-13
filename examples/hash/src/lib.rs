use node_bindgen::derive::node_bindgen;
use std::collections::HashMap;

/// Make simple hash/object with two values/properties
#[node_bindgen]
fn make_hash() -> HashMap<String, bool> {
    let mut hash = HashMap::new();
    hash.insert("foo".to_string(), true);
    hash.insert("bar".to_string(), false);
    hash
}

/// Sum the values of a hash of integers
#[node_bindgen]
fn sum_hash(hash: HashMap<String, i32>) -> i32 { hash.values().sum() }