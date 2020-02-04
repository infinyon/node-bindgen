# node-bindgen

Write native nodejs module using idiomatic rust easy way.


# Example

```rust

use node_bindgen::derive::node_bindgen;

/// generate nodejs sum function
#[node_bindgen]
fn sum(first: i32, second: i32) -> i32 {        
    first + second
}

```

Then can import as nodejs module!


```js
let addon = require('./dylib');

addon.sum(1,2)
3
```


# Build

To build nodejs module, first install nj-cli.

```
cargo instal nj-cli
```

To generate nodejs module:
```
nj-cli build
```

This will generate 'dylib' in the current directory.

