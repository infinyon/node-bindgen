const assert = require('assert');

let addon = require('./dist');
const { format } = require('path');

let array = addon.makeArray(10);

let count = 0;
array.forEach( (val, index)  => {
    assert.equal(val,index,"index value is same as value");
    count = count + 1;
});
assert.equal(count,10);

assert.equal(addon.sumArray([1,2,3]),6,"sum should be 6");

