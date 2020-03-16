use node_bindgen::derive::node_bindgen;

#[node_bindgen]
fn hello<F: Fn(String)>(first: f64, second: F) {
    let msg = format!("argument is: {}", first);

    second(msg);
}