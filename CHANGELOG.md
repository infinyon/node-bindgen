# Changelog

## Unreleased

## [4.1.0] - 2020-12-23

# Improvements

- Support for Dynamic Stream ([#110])
- Enforce Cargo fmt in CI ([#113])

## Fixed
- Fixed multiple mutable borrow isse (#[115])
- Made nj-cli exit non-zero when cargo build fails (#[105])

## [4.0.0] - 2020-11-20

## Improvements
- Support for converting to/from [Rust BigInt][rust-bigint] to/from [JavaScript BigInt][js-bigint] ([#97][]).
- Support for converting Rust `u64` to BigInt in JavaScript. ([#97])
- Updated to N-API v7 in `js-sys` ([#97])

[rust-bigint]: https://crates.io/crates/num-bigint
[js-bigint]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigInt


## Fixed
- Fixed race condition for promise ([#102])

## [3.0.0] - 2020-10-14
- Support for Passing Buffer to Rust
- Support for Env cleanup

# [2.1.0] - 2020-05-15

## Improvements
- Support for Typed Array Buffer ([#24])
- Support for Array ([#26])

## Fixed
- Fixed conversion of `()` ([#31])

# [2.0.0] - 2020-05-011

## Improvements
- Refactor procedure macro ([#21])
- Support optional argument

## Fixed

- Proper support for boolean ([#19])
