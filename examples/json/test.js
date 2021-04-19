const assert = require('assert');

let addon = require('./dist');

assert.deepEqual(addon.customJson(), {
    customFieldName: 10
}, "verify custom json");
assert.deepEqual(addon.standardJson(), {
    someName: "John",
    aNumber: 1337
}, "verify standard json");
assert.deepEqual(addon.multilevelJson(), {
    val: ["hello"]
}, "verify multilevel json");