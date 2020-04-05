# Low level bindings to Nodejs N-API.

Rust wrapper for NodeJs [N-API](https://nodejs.org/api/n-api.html)

The code is check-in in order to avoid dependency to LLVM.

# Manual re-generation of binding.rs

## Prerequisite

This requires [LLVM](https://rust-lang.github.io/rust-bindgen/requirements.html).

Code generation has been tested with LLVM 9 or greater.  Older version may work but not guaranteed.

## Generation

Run following shell command:

```make generate```

Which performs following:
* Install required bindgen execution version
* Generate src/binding.rs using bindgen binary