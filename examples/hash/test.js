const assert = require('assert');

let addon = require('./dist');
const { format } = require('path');

// create a Rust HashMap and return it as a JavaScript object
let obj = addon.makeHash(2);
assert.equal(obj.foo, true);
assert.equal(obj.bar, false);

// create a simple JavaScript object and send it to Rust to sum its properties
let hash = {
    "a": 6,
    "b": 12,
    "c": 24
}
let sum = addon.sumHash(hash);
assert.equal(sum, 42);

// create nested objects and count the leaf properties
let nested = {
    "a": { "x":"foo", "y": "bar" },
    "b": { "z":"baz" }
}
assert.equal(3, addon.raggedLen(nested));

// property values must be of the same type, and match that of the destination HashMap
assert.throws( () => addon.sumHash({"x":"y"}),{
    message: 'invalid type, expected: number, actual: string'
});

