const assert = require('assert');

let addon = require('./dist');

assert.equal(addon.multiply(5),10);