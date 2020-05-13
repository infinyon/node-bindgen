const assert = require('assert');

let addon = require('./dist');

let t = new addon.TestObject();
t.value = 20;
assert.equal(t.value2,20);


assert.equal(addon.simple(5).value2,5);

addon.create(10).then( (test_object) => {
    console.log("test value is %s",test_object.value2);
});