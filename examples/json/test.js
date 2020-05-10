const assert = require('assert');

let addon = require('./dist');

assert.deepEqual(addon.json(), { val: 10},"verify json");