use node_bindgen::derive::node_bindgen;
use node_bindgen::core::{NjError};


#[node_bindgen]
trait AsyncCallback {
    async fn cb() -> bool;
}



#[node_bindgen]
async fn example_await_cb<F: AsyncCallback>(handle: F) {
    let value = handle.cb().await;
}