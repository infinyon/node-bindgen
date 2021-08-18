let addon = require('./dist');
const assert = require('assert');

assert.strictEqual(addon.hello(2), "hello world 2");


console.log("logging tests succeed");
