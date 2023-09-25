const assert = require('assert');

let addon = require('./dist');

assert.equal(addon.double(5), 10);
