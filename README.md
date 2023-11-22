
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

This project uses the v8 of Node N-API.  Please see following [compatibility](https://nodejs.org/api/n-api.html#n_api_n_api_version_matrix) matrix.

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

This is a one time step.

## Configuring Cargo.toml

Add two dependencies to your projects' ```Cargo.toml```.

Add ```node-bindgen``` as a regular dependency (as below):
```
[dependencies]
node-bindgen = { version = "6.0" }
```

Then add ```node-bindgen```'s procedure macro to your build-dependencies as below:
```
[build-dependencies]
node-bindgen = { version = "6.0", default-features = false, features = ["build"] }
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
Welcome to Node.js v18.18.0.
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

## Struct serialization

Structs, including generic structs, can have have the to-JS conversion boilerplate autogenerated.
Just apply the `node_bindgen` macro to your struct:

```rust
#[node_bindgen]
struct MyJson {
    some_name: String,
    a_number: i64
}

#[node_bindgen]
fn my_json() -> MyJson {
    MyJson {
        some_name: "John".to_owned(),
        a_number: 1337
    }
}
```

```js
let addon = require('./dist');
assert.deepStrictEqual(addon.my_json(), {
    someName: "John",
    aNumber: 1337
});
```

Note that the fields must implement
`node_bindgen::core::TryIntoJs` themselves.
Any references must also implement `Clone`.
Field names will be converted to camelCase.

## Enums

Enums will also have their JS representation autogenerated with the help of `node_bindgen`:

```rust
#[node_bindgen]
enum ErrorType {
    WithMessage(String, usize),
    WithFields {
        val: usize
    },
    UnitErrorType
}

#[node_bindgen]
fn with_message() -> ErrorType {
    ErrorType::WithMessage("test".to_owned(), 321)
}

#[node_bindgen]
fn with_fields() -> ErrorType {
    ErrorType::WithFields {
        val: 123
    }
}

#[node_bindgen]
fn with_unit() -> ErrorType {
    ErrorType::UnitErrorType
}
```

```js
assert.deepStrictEqual(addon.withMessage(), {
    withMessage: ["test", 321n]
});
assert.deepStrictEqual(addon.withFields(), {
    withFields: {
        val: 123n
    }
});
assert.deepStrictEqual(addon.withUnit(), "UnitErrorType")
```

Tuple variants will be converted into lists, struct variants converted to objects, and unit variants converted into strings matching the variant's name in PascalCase.
Generics and references are supported, with the same caveats as for structs.

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

Just make sure that you are compiling the rust module using
```
$ npx electron-build-env nj-cli build --release
```

otherwise you will get dreaded  `A dynamic link library (DLL) initialization routine failed` when importing the rust module in electron

## Preparing npm packages

Node module generated with `node-bindgen` can be used directly in any node JS project, just copied `index.node` into it. But in case of direct access to a module IDE will not highlight available functions, classes etc. Usually, this is not comfortable and makes the risks of potential bugs higher as soon as the public API of the node module is changed.

To create a full-fledged npm package with TypeScript types definitions and all necessary JavaScript wrappers can be used a crate `tslink`.

`tslink` crate generates files `*.d.ts`, `*.js` and `package.json` with a description of the npm module. Such package could be integrated into an end-project with minimal effort. 

In addition, because `tslink` generates TypeScript types definitions, any changes on the native node module (`index.node`) will be highlighted by `TypeScript` compiler and it makes the risk of bugs (related to changed API or public data types) much lower.

For example,

```ignore
#[macro_use] extern crate tslink;
use tslink::tslink;
use node_bindgen::derive::node_bindgen;

struct MyScruct {
    inc: i32,
}

#[tslink(class)]
#[node_bindgen]
impl MyScruct {
    #[tslink(constructor)]
    #[node_bindgen(constructor)]
    pub fn new(inc: i32) -> Self {
        Self { inc }
    }

    #[tslink(snake_case_naming)]
    #[node_bindgen]
    fn inc_my_number(&self, a: i32) -> i32 {
        a + self.inc
    }
}
```

Would be represented (`*.d.ts`) as

```ignore
export declare class MyStruct {
    constructor(inc: number);
    incMyNumber(a: number): number;
}
```

Pay your attention, call of `#[tslink]` should be always above of call `#[node_bindgen]`.

Also, please **note**, `node-bindgen` by default applies snake case naming to methods. You should use `#[tslink(snake_case_naming)]` to consider this moment (see more on [crate page](https://docs.rs/tslink/0.1.0/tslink)).

`tslink` requires a configuration file in the root of your project (`tslink.toml`). Configuration file should include a valid path to the native node module. By default `node-bindgen` creates `index.node` in `./dist` folder of your `root`.

File: `./tslink.toml` (in a `root` of project):

```ignore
node = "./dist/index.node"
```

Full example of usage `tslink` and `node-bindgen` is [here](https://github.com/DmitryAstafyev/tslink/tree/master/examples/node_bindgen).

See more API documentation on a `tslink` [crate page](https://docs.rs/tslink/0.1.0/tslink).

## Contributing

If you'd like to contribute to the project, please read our [Contributing guide](CONTRIBUTING.md).

## License

This project is licensed under the [Apache license](LICENSE-APACHE).
