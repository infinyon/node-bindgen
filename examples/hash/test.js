const assert = require('assert');

let addon = require('./dist');
const { format } = require('path');


let obj = addon.makeHash(2);
assert.equal(obj.foo, true);
assert.equal(obj.bar, false);

let hash = {
    "a": 6,
    "b": 12,
    "c": 24
}

let sum = addon.sumHash(hash);
assert.equal(sum, 42);

assert.throws( () => addon.sumHash({"x":"y"}),{
    message: 'invalid type, expected: number, actual: string'
});

