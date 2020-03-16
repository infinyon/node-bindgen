
use node_bindgen::derive::node_bindgen;
use node_bindgen::core::NjError;


#[node_bindgen]
fn hello(count: i32) -> String {        
    format!("hello world {}", count)
}


#[node_bindgen]
fn sum(first: i32, second: i32) -> i32 {        
    first + second
}

// throw error if first > second, otherwise return sum
#[node_bindgen]
fn min_max(first: i32, second: i32) -> Result<i32,NjError> {        
    if first > second {
        println!("throwing error");
        Err(NjError::Other("first arg is greater".to_owned()))
    } else {
        Ok(first + second )
    }
}

