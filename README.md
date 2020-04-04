# node-bindgen
[![Build Status](https://travis-ci.com/palfrey/node-bindgen.svg?branch=master)](https://travis-ci.com/palfrey/node-bindgen)

Easy way to write native Node.js module using idiomatic rust.


# Example

```rust

use node_bindgen::derive::node_bindgen;

/// generate nodejs sum function
#[node_bindgen]
fn sum(first: i32, second: i32) -> i32 {        
    first + second
}

```

Import as Node.js module!


```js
let addon = require('./dylib');

addon.sum(1,2)
3
```

# N-API

This crates utilizes Node N-API.

# Build

Use nj-cli to generate native module.  First install:

```
cargo install nj-cli
```

Build command to generate module directory.

```
nj-cli build
```

This will generates ".dylib" folder.


# More examples

##  Callback

Use rust closure to invoke JS callback

```rust
#[node_bindgen]
fn hello<F: Fn(String)>(first: f64, second: F) {

    let msg = format!("argument is: {}", first);

    second(msg);
}
```

from node:

```js
let addon = require('./dylib');

addon.hello(2,function(msg){
  assert.equal(msg,"argument is: 2");
  console.log(msg);  // print out argument is 2
});

```

## Async functions

Async rust function will return as promise.  

```rust

use std::time::Duration;
use flv_future_core::sleep;
use node_bindgen::derive::node_bindgen;


#[node_bindgen]
async fn hello(arg: f64) -> f64 {
    println!("sleeping");
    sleep(Duration::from_secs(1)).await;
    println!("woke and adding 10.0");
    arg + 10.0
}
```

```js
let addon = require('./dylib');

addon.hello(5).then((val) => {
  console.log("future value is %s",val);
});

```

## JavaScript class

JavaScript class can be implemented easily.

```rust

struct MyClass {
    val: f64,
}


#[node_bindgen]
impl MyClass {

    #[node_bindgen(constructor)]
    fn new(val: f64) -> Self {
        Self { val }
    }

    #[node_bindgen]
    fn plus_one(&self) -> f64 {
        self.val + 1.0
    }

    #[node_bindgen(getter)]
    fn value(&self) -> f64 {
        self.val
    }
}
```

```js
let addon = require('./dylib');
const assert = require('assert');

let obj = new addon.MyObject(10);
assert.equal(obj.value,10,"verify value works");
assert.equal(obj.plusOne(),11);
```


