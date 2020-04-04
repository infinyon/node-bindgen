# Low level bindings to Nodejs N-API.

Rust wrapper for NodeJs [N-API](https://nodejs.org/api/n-api.html)

# Manual re-generation of binding.rs

```make generate```

This will generate src/binding.rs.  The code is check-in in order to avoid dependency to LLVM which is required by bindgen crate.