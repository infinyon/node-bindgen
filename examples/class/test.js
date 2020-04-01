const assert = require('assert');

let addon = require('./dist');

let obj = new addon.MyObject(10);
assert.equal(obj.value,10,"verify value works");
assert.deepEqual(obj.json, { val: 10},"verify json");
assert.equal(obj.plusOne(),11);

let obj2 = obj.multiply(-1);
assert.equal(obj2.value,-10);

obj.changeValue(100);
assert.equal(obj.value,100);

obj.value2(50);
assert.equal(obj.value,50);

obj.value3 = 10;
assert.equal(obj.value,10,"test setter");

obj.value4 = 60;
assert.equal(obj.value,60,"test test setter with custom property");

obj.value4 = -10;
assert.equal(obj.isPositive,false);

obj.value4 = 10;
assert.equal(obj.isPositive,true);

obj.clear = false;
assert.equal(obj.value,10);

obj.clear = true;
assert.equal(obj.value,0);

assert.equal(obj.plusScore( { score: 10 }),10);




let t = new addon.TestObject();
t.value = 20;
assert.equal(t.value2,20);

// test passing of test object
assert.equal(obj.plusTest(t),20);

obj.plusTwo(10).then( (val) => {
    console.log("plus two is ",val);
});

obj.multiply2(-1).then( (obj3) => {
    console.log("multiply two ",obj3.value);
});

obj.sleep((msg) => {
    assert.equal(msg,"hello world");;
});


addon.create(10).then( (test_object) => {
    console.log("test value is {}",test_object.value2);
});