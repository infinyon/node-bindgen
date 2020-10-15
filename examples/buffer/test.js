const assert = require('assert');

let addon = require('./dist');
const { format } = require('path');


let bytes = addon.test(5);
console.log("received bytes: ", bytes.byteLength);

// create buffer view from byte array
let buffer = Buffer.from(bytes);
assert.deepStrictEqual(JSON.parse(buffer), { a: 'b', b: 5 });


let record = addon.test2(10);
assert.strictEqual(record.comment, "array buffer is cool!");
assert.deepStrictEqual(JSON.parse(Buffer.from(record.buffer)), { a: 'b', b: 10 });


assert.strictEqual(addon.test3(Buffer.from("hello")),5);
