use node_bindgen::derive::node_bindgen;
use node_bindgen::core::{NjError};

struct ThreadSafeThunk {
    thunk: Fn(String)
}

#[node_bindgen]
impl ThreadSafeThunk {
    
    #[node_bindgen(constructor)]
    fn new(thunk: Fn(String)) -> Self {
        Self {
            thunk
        }
    }

    async fn call_thunk(&self) -> Result<(), NjError> {
        "world".to_string();

        Ok(())
    }
}
