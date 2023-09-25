const assert = require('assert');

let addon = require('./dist');

console.print(addon.multiply(5.0));
assert.equal(addon.multiply(5), 10.0);
