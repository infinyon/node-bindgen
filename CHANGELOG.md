# Changelog

## Unreleased

## [4.0.0] - 2020-11-20-2020

## Improvements
- Support for converting to/from [Rust BigInt][rust-bigint] to/from [JavaScript BigInt][js-bigint] ([#97][]).
- Support for converting Rust `u64` to BigInt in JavaScript. ([#97][])
- Updated to N-API v7 in `js-sys`. ([#97][])

[rust-bigint]: https://crates.io/crates/num-bigint
[js-bigint]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigInt
[#97]: https://github.com/infinyon/node-bindgen/pull/97

## Fixed
- Fixed race condition for promise [#101] https://github.com/infinyon/node-bindgen/pull/102

## [3.0.0] - 2020-10-14-2000
- Support for Passing Buffer to Rust
- Support for Env cleanup

# [2.1.0] - 2020-05-15

## Improvements
- Support for Typed Array Buffer [#24](https://github.com/infinyon/node-bindgen/pull/24)
- Support for Array [#26](https://github.com/infinyon/node-bindgen/pull/26)

## Fixed
- Fixed conversion of `()` [#31](https://github.com/infinyon/node-bindgen/pull/31)

# [2.0.0] - 2020-05-011

## Improvements
- Refactor procedure macro [#21](https://github.com/infinyon/node-bindgen/pull/21)
- Support optional argument

## Fixed

- Proper support for boolean [#19](https://github.com/infinyon/node-bindgen/pull/19)
