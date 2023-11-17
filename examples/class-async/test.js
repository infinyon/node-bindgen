const assert = require('assert');

let addon = require('./dist');

let obj = new addon.MyObject(10);


obj.plusTwo(10).then((val) => {
    console.log("plus two is ", val);
});

obj.multiply2(-1).then((obj3) => {
    console.log("multiply two ", obj3.value);
});

obj.sleep((msg) => {
    assert.equal(msg, "hello world");;
});
