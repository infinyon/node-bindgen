const assert = require('assert');

let addon = require('./dist');

const hugeBin = BigInt("0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF")

// Test that we can go back and forth through the FFI
assert.equal(addon.doNothing(hugeBin), hugeBin);

// Test out some signage
assert.equal(addon.goNegative(hugeBin), hugeBin*BigInt(-1));

// How about multiplication?
assert.equal(addon.multiplyBigInt(hugeBin), hugeBin*(BigInt(2)));

// Basic tests from js-env
assert.equal(addon.multiplyBigInt(BigInt(-5)), BigInt(-10));
assert.equal(addon.multiplyBigInt(BigInt(5)), BigInt(10));
assert.equal(addon.multiplyBigInt(BigInt(5)), BigInt(10));

// Rust's u64s turn into JS BigInts.
assert.equal(addon.returnU64(5), BigInt(5));
