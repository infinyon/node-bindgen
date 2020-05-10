const assert = require('assert');

let addon = require('./dist');

assert.equal(addon.add(1),10,"verify integer");
assert.equal(addon.add({ val: 1}),10,"verify json");
assert.equal(addon.add(),0,"verify no argument");