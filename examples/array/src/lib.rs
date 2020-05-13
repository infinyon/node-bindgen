

use node_bindgen::derive::node_bindgen;



/// create array and fill with increase value
#[node_bindgen]
fn make_array(count: i32) -> Vec<i32> {

    let mut array = vec![];
    for i in 0..count {
        array.push(i);
    }
    array
}


/// sum array of values
#[node_bindgen]
fn sum_array(array: Vec<i32>) -> i32 {

   array.iter().sum()
}

