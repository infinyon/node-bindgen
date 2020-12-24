use node_bindgen::derive::node_bindgen;

#[node_bindgen]
fn hello<F: Fn(String)>(first: f64, second: F) {
    let msg = format!("argument is: {}", first);

    second(msg);
}

#[node_bindgen]
fn example<F: Fn(i32)>(cb: F, second: i32) {
    cb(second * 2)
}

/*
#[node_bindgen]
fn sum<F: Fn(i32) -> String>(cb: F,second: i32) -> String {
    let message = cb(second*2);
    format!("my message: {}",message)
}
*/
