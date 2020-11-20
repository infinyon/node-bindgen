
<h1 align="center">node-bindgen</h1>
<div align="center">
 <strong>
   Easy way to write native Node.js module using idiomatic Rust
 </strong>
</div>

<br />

<div align="center">
   <!-- CI status -->
  <a href="https://github.com/infinyon/node-bindgen/actions">
    <img src="https://github.com/infinyon/node-bindgen/workflows/CI/badge.svg"
      alt="CI Status" />
  </a>
  <!-- Crates version -->
  <a href="https://crates.io/crates/node-bindgen">
    <img src="https://img.shields.io/crates/v/node-bindgen?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/node-bindgen">
    <img src="https://img.shields.io/crates/d/node-bindgen.svg?style=flat-square"
      alt="Download" />
  </a>
  <!-- docs.rs docs -->
  <a href="https://docs.rs/node-bindgen">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>

  <a href="https://discord.gg/V5MhmEq">
    <img src="https://img.shields.io/discord/695712741381636168.svg?logo=discord&style=flat-square"
      alt="chat" />
  </a>
</div>


## Features

- __Easy:__ Just write idiomatic Rust code, node-bindgen take care of generating Node.js FFI wrapper codes.
- __Safe:__ Node.js arguments are checked automatically based on Rust types.
- __Async:__ Support Async Rust.  Async codes are translated into Node.js promises.
- __Class:__ Rust struct can be accessed using Node.js classes.
- __Stream:__ Implement Node.js stream using Rust
- __N-API:__ Use Node.js N-API, which means you don't have to recompile your module. 

# Compatibility with Node.js version

This project uses the v7 of Node N-API.  Please see following [compatibility](https://nodejs.org/api/n-api.html#n_api_n_api_version_matrix) matrix.

Following OS are supported:
* Linux
* MacOs
* Windows



# Why node-bindgen?

Writing native node-js requires lots of boilerplate code.  Node-bindgen generates external "C" glue code from rust code, including native module registration.  node-bindgen make it writing node-js module easy and fun.  


# Getting started

## CLI Installation

Install nj-cli command line, which will be used to generate the native library.

```
cargo install nj-cli
```

This is one time step.

## Configuring Cargo.toml

Add two dependencies to your projects' ```Cargo.toml```.  

Add ```node-bindgen``` as a regular dependency (as below):
```
[dependencies]
node-bindgen = { version = "3.0.0" }
```

Then add ```node-bindgen```'s procedure macro to your build-dependencies as below:
```
[build-dependencies]
node-bindgen = { version = "3.0.0", features = ["build"] }
```

Then update crate type to ```cdylib``` to generate node.js compatible native module:
```
[lib]
crate-type = ["cdylib"]
```

Finally, add ```build.rs``` at the top of the project with following content:

```
fn main() {
    node_bindgen::build::configure();
}
```


# Example

Here is a function that adds two numbers.  Note that you don't need to worry about JS conversion.


```rust

use node_bindgen::derive::node_bindgen;

/// add two integer
#[node_bindgen]
fn sum(first: i32, second: i32) -> i32 {        
    first + second
}

```

## Building native library

To build node.js library, using ```nj-cli``` to build:

```
nj-cli build
```

This will generate Node.js module in "./dist" folder.

To build a release version:
```
nj-cli build --release
```

## Watching `./src` for Changes

While developing your native module, you may want to watch for file changes and run a command when a change occurs, for example `cargo check` or `cargo build`.

For this, we can use `nj-cli watch`. 

`nj-cli watch` installs <small>[if it does not exist]</small> and passes arguments to [`cargo watch`](https://crates.io/crates/cargo-watch). By default, `nj-cli watch` will run `cargo check` against your `./src` files.

To see all available methods for `nj-cli watch`, run the following command:

> `nj-cli watch -- --help`

## Using in Node.js

Then in the Node.js, rust function can be invoked as normal node.js function:

```js
$ node
Welcome to Node.js v14.0.0.
Type ".help" for more information.
> let addon = require('./dist');
undefined
> addon.sum(2,3)
5
> 
```


# Features

## Function name or method can be renamed instead of default mapping

```rust
#[node_bindgen(name="multiply")]
fn mul(first: i32,second: i32) -> i32 {        
    first * second 
}
```

Rust function mul is re-mapped as ```multiply```

## Optional argument

Argument can be skipped if it is marked as optional
```rust
#[node_bindgen]
fn sum(first: i32, second: Option<i32>) -> i32 {        
    first + second.unwrap_or(0)
}
```
Then sum can be invoked as
```sum(10)``` or ```sum(10,20)```


##  Callback

JS callback are mapped as Rust closure.

```rust
#[node_bindgen]
fn hello<F: Fn(String)>(first: f64, second: F) {

    let msg = format!("argument is: {}", first);

    second(msg);
}
```

from node:

```js
let addon = require('./dist');

addon.hello(2,function(msg){
  assert.equal(msg,"argument is: 2");
  console.log(msg);  // print out argument is 2
});
```

Callback are supported in Async rust as well.

## Support for Async Rust

Async rust function is mapped to Node.js promise.

```rust

use std::time::Duration;
use flv_future_aio::time::sleep;
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
let addon = require('./dist');

addon.hello(5).then((val) => {
  console.log("future value is %s",val);
});

```


## JavaScript class

JavaScript class is supported.

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
let addon = require('./dist');
const assert = require('assert');

let obj = new addon.MyObject(10);
assert.equal(obj.value,10,"verify value works");
assert.equal(obj.plusOne(),11);
```

There are more features in the examples folder.

## Windows + Electron Support
When using node-bindgen with electron on Windows, `nj-build` must
compile a C++ file, `win_delay_load_hook.cc`, and therefore it is required that the development
environment has a valid C/C++ compiler. 

> If your machine does not have a valid C/C++ compiler, install [Microsoft VSCode](https://code.visualstudio.com/docs/cpp/config-mingw).

In the future, this file will be re-written in Rust, removing this dependency.

## Contributing

If you'd like to contribute to the project, please read our [Contributing guide](CONTRIBUTING.md).

## License

This project is licensed under the [Apache license](LICENSE-APACHE).
