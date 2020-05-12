const assert = require('assert');

let addon = require('./dist');
const { format } = require('path');

let bytes = addon.test(5);
console.log("received bytes: ",bytes.byteLength);

// create buffer view from byte array
let buffer = Buffer.from(bytes);
assert.deepEqual(JSON.parse(buffer), { a: 'b', b: 5});


let record = addon.test2(10);
assert.equal(record.comment,"array buffer is cool!");
assert.deepEqual(JSON.parse(Buffer.from(record.buffer)), { a: 'b', b: 10});

