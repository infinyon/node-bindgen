use node_bindgen::derive::node_bindgen;
use node_bindgen::core::bigint::BigInt;

// example where we multiply a big_int.
#[node_bindgen]
fn multiply_big_int(arg: BigInt) -> BigInt {
    println!("bigint arg: {arg}");
    arg * 2
}

// Test that we can go across the FFI without screwing up the bits.
#[node_bindgen]
fn do_nothing(arg: BigInt) -> BigInt {
    println!("bigint arg: {arg}");
    arg
}

// Test out the signage conversion
#[node_bindgen]
fn go_negative(arg: BigInt) -> BigInt {
    println!("bigint arg: {arg}");
    -1 * arg
}

// Test out that we can return a u64 which is automatically converted to a bigint.
#[node_bindgen]
fn return_u64(arg: u32) -> u64 {
    println!("bigint arg: {arg}");
    arg as u64
}
