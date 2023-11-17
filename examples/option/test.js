const assert = require('assert');

let addon = require('./dist');

assert.equal(addon.test(100, 200), 300);

assert.equal(addon.test(100), 100);

assert.equal(addon.test(undefined, 200), 200);

assert.equal(addon.test(null, 200), 200);

assert.equal(addon.test(100, undefined), 100);

assert.equal(addon.test(100, null), 100);

assert.equal(addon.test(undefined, undefined), 1);

assert.equal(addon.test(null, null), 1);
