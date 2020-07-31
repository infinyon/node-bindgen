const assert = require('assert');

let addon = require('./dist');


let obj = new addon.MyObject(10, 2);


assert.strictEqual(obj.value, 10, "verify value works");


assert.strictEqual(obj.plusOne(), 11);
assert.strictEqual(obj.value2, 2);


obj.changeValue(100);
assert.strictEqual(obj.value, 100);


obj.updateValue(50);
assert.strictEqual(obj.value, 50);

obj.value3 = 10;
assert.strictEqual(obj.value, 10, "test setter");

obj.value4 = 60;
assert.strictEqual(obj.value, 60, "test test setter with custom property");

obj.value4 = -10;
assert.strictEqual(obj.isPositive, false);


obj.value4 = 10;
assert.strictEqual(obj.isPositive, true);

obj.clear = false;
assert.strictEqual(obj.value, 10);

obj.clear = true;
assert.strictEqual(obj.value, 0);


console.log("class simple test succeed");
