use node_bindgen::derive::node_bindgen;

#[node_bindgen]
fn test(a: Option<i32>, b: Option<i32>) -> i32 {
    if let (Some(a), Some(b)) = (a, b) {
        a + b
    } else {
        a.unwrap_or(b.unwrap_or(1))
    }
}
