# Changelog

## Unreleased
### Improvements
- Add cross compilation support to nj-cli. ([#182](https://github.com/infinyon/node-bindgen/pull/182)).

## [5.0.0] - 2021-07-15
### Improvements
- Added support for automatic conversion of structs and enums into the JS representation by decorating their definition with `#[node_bindgen]` ([#148](https://github.com/infinyon/node-bindgen/pull/148) and [#155](https://github.com/infinyon/node-bindgen/pull/155))
- Defined a `NjError::Native` Error payload, which allows errors to return structured data to JS
- `Result<T, E>` converts the error value to JS using `TryIntoJs` for structured error payloads
- Add support for passing tuples between Node and Rust ([#142](https://github.com/infinyon/node-bindgen/pull/142))
- Bump electron from 9.3.1 to 9.4.0 in /examples/electron ([#135](https://github.com/infinyon/node-bindgen/pull/135))
- Update JSArrayBuffer to be usable in `env.convert_to_rust` ([#136](https://github.com/infinyon/node-bindgen/pull/136))
- Added missing unsigned and signed integer conversions to JS [#158](https://github.com/infinyon/node-bindgen/pull/158)
- `serde_json` is automatically serialized to it's JS representation ([#159](https://github.com/infinyon/node-bindgen/pull/159))
- `uuid`s are automatically serialized to JS ([#160](https://github.com/infinyon/node-bindgen/pull/160))

## [4.3.0] - 2021-03-13
### Improvements
- update trybuild to point to infinyon repo
- update dependencies
- Bump dependency on nj-core to 4.1.3
- Rename `to_js` to `into_js` in `pub trait IntoJs`

## [4.2.2] - 2021-01-06
### Fixed
- Fix lifetime in `JSValue` for `&str`.

## [4.2.1] - 2020-12-29
### Improvements
- Implement `JSValue` for `&str` ([#126](https://github.com/infinyon/node-bindgen/pull/126))
- Add lifetime support for procedural macro ([#127](https://github.com/infinyon/node-bindgen/pull/127))

## [4.1.1] - 2020-12-29

### Improvements
- Implement `JSArrayBuffer` with managed lifecycle of of `ArrayBuffer` directly ([#121](https://github.com/infinyon/node-bindgen/pull/121))
- Add `impl<T> TryIntoJs for Option<T> where T: TryIntoJs` ([#122](https://github.com/infinyon/node-bindgen/pull/122))

### Fixed
- Fix `arm64` builds ([#120](https://github.com/infinyon/node-bindgen/pull/120))

## [4.1.0] - 2020-12-23

### Improvements
- Support for Dynamic Stream ([#110](https://github.com/infinyon/node-bindgen/pull/110))
- Enforce Cargo fmt in CI ([#113](https://github.com/infinyon/node-bindgen/pull/113))

### Fixed
- Fixed multiple mutable borrow isse (#[115])
- Made nj-cli exit non-zero when cargo build fails (#[105])

## [4.0.0] - 2020-11-20

### Improvements
- Support for converting to/from [Rust BigInt][rust-bigint] to/from [JavaScript BigInt][js-bigint] ([#97](https://github.com/infinyon/node-bindgen/pull/97)).
- Support for converting Rust `u64` to BigInt in JavaScript. ([#97](https://github.com/infinyon/node-bindgen/pull/97))
- Updated to N-API v7 in `js-sys` ([#97](https://github.com/infinyon/node-bindgen/pull/97))

[rust-bigint]: https://crates.io/crates/num-bigint
[js-bigint]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigInt

### Fixed
- Fixed race condition for promise ([#102](https://github.com/infinyon/node-bindgen/pull/102))

## [3.0.0] - 2020-10-14
- Support for Passing Buffer to Rust
- Support for Env cleanup

## [2.1.0] - 2020-05-15

### Improvements
- Support for Typed Array Buffer ([#24]((https://github.com/infinyon/node-bindgen/pull/24)))
- Support for Array ([#26](https://github.com/infinyon/node-bindgen/pull/26))

### Fixed
- Fixed conversion of `()` ([#31]((https://github.com/infinyon/node-bindgen/pull/31)))

## [2.0.0] - 2020-05-011

### Improvements
- Refactor procedure macro ([#21]((https://github.com/infinyon/node-bindgen/pull/21)))
- Support optional argument

### Fixed
- Proper support for boolean ([#19]((https://github.com/infinyon/node-bindgen/pull/19)))
